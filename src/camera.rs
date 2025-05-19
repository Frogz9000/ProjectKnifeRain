use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::FRAC_PI_2;
use bevy::{
    input::{ mouse::AccumulatedMouseMotion}, prelude::*, render::view::RenderLayers
};

pub struct CameraControls;
impl Plugin for CameraControls{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,(
            setup_camera,
        ));
        app.add_systems(Update,(update_camera_mouse_event,update_pov,grab_mouse));
    }
}

#[derive(Component)]
struct PlayerCamera;
#[derive(Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);
impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))//arbitrary value, add settings controller later
    }
}
#[derive(Component)]
struct WorldCamera;

const DEFAULT_RENDER_LAYER: usize = 0;
const VIEWMODEL_RENDER_LAYER: usize = 1;

fn setup_camera(
    mut commands: Commands
){
   commands.spawn((
        PlayerCamera,
        CameraSensitivity::default(),
        Transform::from_xyz(25.0,1.0,25.0),
        Visibility::default(),
   )).with_children(|parent| {
    //spawn world camera as child: mut fov 90 
    parent.spawn((
        WorldCamera,
        Camera3d::default(),
        Projection::from(PerspectiveProjection{
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
    ));
    //spwan view model camera as child: immut fov 70 may change to depending on view model generated
    parent.spawn((
        Camera3d::default(),
        Camera{
            order: 1,
            ..default()
        },
        Projection::from(PerspectiveProjection{
            fov: 70.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::layer(VIEWMODEL_RENDER_LAYER),
    ));
    //Add parent.spawn viewmodel when ready
   });
}

fn update_camera_mouse_event(
    accum_mouse_motion: Res<AccumulatedMouseMotion>,
    camera: Single<(&mut Transform, &CameraSensitivity), With<PlayerCamera>>
){
    let (mut transform, camera_sensitivity) = camera.into_inner();
    let delta = accum_mouse_motion.delta;
    if delta != Vec2::ZERO{ //if there was net mouse movement
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw,pitch,roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;
        //prevent camera from going fully up or down to prevent ambiguity of what forward is/reversing yaw
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll)
    }
}
//for now FOV will be controlled with up/down arrow keys for development
fn update_pov(
    input: Res<ButtonInput<KeyCode>>,
    mut world_projection: Single<&mut Projection, With<WorldCamera>>
){
    let Projection::Perspective(perspective) = world_projection.as_mut() else{
        unreachable!();
    };
    if input.pressed(KeyCode::ArrowUp){
        perspective.fov -= 1.0_f32.to_radians();
        perspective.fov = perspective.fov.max(20.0_f32.to_radians());//prevent lower than 20 fov
    }
    if input.pressed(KeyCode::ArrowDown){
        perspective.fov += 1.0_f32.to_radians();
        perspective.fov = perspective.fov.min(160.0_f32.to_radians());//prevent higher than 160 fov
    }
}

fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}