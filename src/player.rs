use bevy::{math::VectorSpace, window::CursorGrabMode};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody, Velocity};
use std::f32::consts::FRAC_PI_2;
use bevy::{
    input::{ mouse::AccumulatedMouseMotion}, prelude::*, render::view::RenderLayers
};
use crate::camera::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,(
            setup_player,
        ));
        app.add_systems(Update,(
            update_player_keyboard_event,
            update_player_mouse_event,
        ));
        app.add_plugins(CameraControls);
    }
}
#[derive(Component)]
pub struct Player;
//create public struct components to share between camera and player 
#[derive(Component)]
pub struct PlayerPosition(pub Vec3);
#[derive(Component)]
pub struct PlayerLookAngles{
    pub pitch: f32,
    pub yaw: f32,
}
#[derive(Component)]
pub struct Speed(pub f32);
#[derive(Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);
impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))//arbitrary value, add settings controller later
    }
}


//spawn in player rigid body and collider
//need component tag to query in camera
fn setup_player(
    mut commands: Commands
){
    commands.spawn((
        Player,
        Speed(4.0),
        CameraSensitivity::default(),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),//default player hitbox for now
        Transform::from_xyz(25.0, 1.0, 25.0),
        PlayerPosition(Vec3 { x: (25.0), y: (1.0), z: (25.0) }),
        PlayerLookAngles{yaw:0.0,pitch:0.0},
    ));
}

fn update_player_keyboard_event(
    mut player: Query<(&Speed,&mut Transform, &mut PlayerPosition), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    let (speed, mut transform, mut pos) = player.single_mut().unwrap();
    let mut movement = Vec3::ZERO;
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
    //let rot = Quat::from_rotation_y(yaw.0);
    //let angled_move = rot * movement.normalize_or_zero();
    //velocity.linvel.x = angled_move.x * speed.0;
    //velocity.linvel.z  =angled_move.z * speed.0;

    let forward = transform.forward();
    let right = transform.right();
    let movement_relative = (right* movement.x + forward * movement.z).normalize_or_zero();
    transform.translation += movement_relative*speed.0*time.delta_secs();
    pos.0 = transform.translation;
}

fn update_player_mouse_event(
    accum_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform,&mut PlayerLookAngles, &CameraSensitivity), With<Player>>,
){
    let (mut transform,mut look_angle, camera_sensitivity) = player.single_mut().unwrap();//should add option handling but just unwrap for now
    let delta = accum_mouse_motion.delta;
    if delta != Vec2::ZERO{ //if there was net mouse movement
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw,pitch,roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let update_yaw = yaw+delta_yaw;
        //prevent camera from going fully up or down to prevent ambiguity of what forward is/reversing yaw
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let update_pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, update_yaw, pitch, roll);//apply yaw change to hitbox
        //store camera changes to struct
        look_angle.pitch = update_pitch;
        look_angle.yaw = update_yaw;
    }
}