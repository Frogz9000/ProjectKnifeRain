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
        ));
        app.add_plugins(CameraControls);
    }
}
#[derive(Component)]
pub struct Player;
//create public struct components to share between camera and player 
#[derive(Component)]
pub struct Yaw(pub f32);
#[derive(Component)]
pub struct Speed(pub f32);


//spawn in player rigid body and collider
//need component tag to query in camera
fn setup_player(
    mut commands: Commands
){
    commands.spawn((
        Player,
        Yaw(0.0),
        Speed(4.0),
        RigidBody::Dynamic,
        Collider::capsule_y(0.9, 0.3),//default player hitbox for now
        Transform::from_xyz(25.0, 0.0, 25.0),
    ));
}

fn update_player_keyboard_event(
    mut player: Query<(&Yaw,&Speed,&mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
){
    let (yaw, speed, mut velocity) = player.single_mut().unwrap();
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
    let rot = Quat::from_rotation_y(yaw.0);
    let angled_move = rot * movement.normalize_or_zero();
    velocity.linvel.x = angled_move.x * speed.0;
    velocity.linvel.z  =angled_move.z * speed.0;

    //let forward = transform.forward();
    //let right = transform.right();
    //let movement_relative = (right* movement.x + forward * movement.z).normalize_or_zero();
    //transform.translation += movement_relative*speed*time.delta_secs();
}