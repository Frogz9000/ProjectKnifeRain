use bevy::window::CursorGrabMode;
use bevy_rapier3d::{prelude::{Collider, KinematicCharacterController, RigidBody}};
use std::f32::consts::FRAC_PI_2;
use crate::player::*;
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
pub struct PlayerCamera;
#[derive(Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);
impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))//arbitrary value, add settings controller later
    }
}
#[derive(Component)]
struct WorldCamera;

const VIEWMODEL_RENDER_LAYER: usize = 1;
fn setup_camera(
    mut commands: Commands
){
   commands.spawn((
        PlayerCamera,
        CameraSensitivity::default(),
        Transform::from_xyz(25.0,1.6,25.0),
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

//mouse events will be handled by camera, with change of yaw of player hitbox will update
//but change in pitch will not update player so looking up/down does not tip player over
fn update_camera_mouse_event(
    accum_mouse_motion: Res<AccumulatedMouseMotion>,
    mut q_camera: Query<(&mut Transform, &CameraSensitivity), With<PlayerCamera>>,
    mut q_yaw: Query<&mut Yaw, With<Player>>,
){
    let (mut transform, camera_sensitivity) = q_camera.single_mut().unwrap();//should add option handling but just unwrap for now
    let mut current_yaw = q_yaw.single_mut().unwrap();
    let delta = accum_mouse_motion.delta;
    if delta != Vec2::ZERO{ //if there was net mouse movement
        current_yaw.0 -= delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw,pitch,roll) = transform.rotation.to_euler(EulerRot::YXZ);
        
        //prevent camera from going fully up or down to prevent ambiguity of what forward is/reversing yaw
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, current_yaw.0, pitch, roll)
    }
}
//replace this with a query of player transform to get movement
//move this function to player
fn update_camera_keyboard_event(
    camera: Single<&mut Transform, With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    let mut transform = camera.into_inner();
    let mut movement = Vec3::ZERO;
    let speed = 4.0; //player speed modifier, add query var for this later, maybe item?
    if input.pressed(KeyCode::KeyW){
        movement.z += 1.0;
    }
    if input.pressed(KeyCode::KeyS){
        movement.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyA){
        movement.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD){
        movement.x += 1.0;
    }
    let forward = transform.forward();
    let right = transform.right();
    let movement_relative = (right* movement.x + forward * movement.z).normalize_or_zero();
    transform.translation += movement_relative*speed*time.delta_secs();
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