use bevy::window::CursorGrabMode;
use std::f32::consts::FRAC_PI_2;
use crate::player::*;
use bevy::{
    prelude::*, render::view::RenderLayers
};

pub struct CameraControls;
impl Plugin for CameraControls{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,(
            setup_camera,
        ));
        app.add_systems(Update,(update_camera,update_pov,grab_mouse));
    }
}
#[derive(Component)]
pub struct PlayerCamera;
#[derive(Component)]
struct WorldCamera;

const VIEWMODEL_RENDER_LAYER: usize = 1;
const CAMERA_OFFSET_Z: f32 = 0.0;//apply to camera to lag behind hitbox for debug, set to 0 for first person
const CAMERA_OFFSET_Y: f32 = 0.5;//height offset to have camera at a certain level of player hitbox, not bottom of hitbox
fn setup_camera(
    mut commands: Commands
){
   commands.spawn((
        PlayerCamera,
        Transform::from_xyz(25.0,CAMERA_OFFSET_Y,25.0+CAMERA_OFFSET_Z),
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

//instead of having camera controls have the camera react to the player
fn update_camera(
    player: Query<(&PlayerPosition,&PlayerLookAngles),With<Player>>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
){
    let (position_vec, angles_struct) = player.single().unwrap();
    let mut transform = camera.single_mut().unwrap();
    let mut camera_offset_position = position_vec.0;
    camera_offset_position.z += CAMERA_OFFSET_Z;
    camera_offset_position.y += CAMERA_OFFSET_Y;
    transform.translation = camera_offset_position;
    //
    let (_,current_pitch,_) = transform.rotation.to_euler(EulerRot::YXZ);
    //prevent camera from going fully up or down to prevent ambiguity of what forward is/reversing yaw
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    let update_pitch = (current_pitch + angles_struct.pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, angles_struct.yaw, update_pitch, 0.0);
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