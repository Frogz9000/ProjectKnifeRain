use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody, Velocity};
use bevy::{
    input::{ mouse::AccumulatedMouseMotion}, prelude::*
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
            sync_player_camera_pos
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


fn setup_player(
    mut commands: Commands
){
    commands.spawn((
        Player,
        Speed(400.0),//adjust as needed for base speed, maybe have items or other modify later: 100 ~ 1m/s
        CameraSensitivity::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.3),//default player hitbox for now
        LockedAxes::ROTATION_LOCKED,//prevent physics induced rotation, manual rotation done from input
        Transform::from_xyz(25.0, 1.0, 25.0),
        PlayerPosition(Vec3 { x: (25.0), y: (1.0), z: (25.0) }),
        PlayerLookAngles{yaw:0.0,pitch:0.0},
        Velocity::zero(),
    ));
}

fn update_player_keyboard_event(
    mut player: Query<(&Speed,&mut Velocity), With<Player>>,
    camera: Query<&Transform,With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
){
    let (speed, mut velocity) = player.single_mut().unwrap();
    let camera_transform = camera.single().unwrap();
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW){
        direction += *camera_transform.forward();
    }
    if input.pressed(KeyCode::KeyS){
        direction += *camera_transform.back();
    }
    if input.pressed(KeyCode::KeyA){
        direction += *camera_transform.left();
    }
    if input.pressed(KeyCode::KeyD){
        direction += *camera_transform.right();
    }
    //flatten vector (ignore y)
    direction.y = 0.0;
    let direction = direction.normalize_or_zero();
    let mut current_speed = speed.0;
    //check for sprint
    if input.pressed(KeyCode::ShiftLeft){
        current_speed = speed.0 * 2.0;//for now double speed when sprinting consider changing to var that can change from gear
    }
    //need mult delta time to get frame independance
    velocity.linvel = Vec3::new(
        direction.x * current_speed* time.delta_secs(),
        velocity.linvel.y,
        direction.z * current_speed * time.delta_secs(),
    )
}

fn sync_player_camera_pos(
    mut query: Query<(&Transform, &mut PlayerPosition), With<Player>>,
){
    let (transform, mut pos) = query.single_mut().unwrap();
    pos.0 = transform.translation;//update position value for camera
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
        transform.rotation = Quat::from_euler(EulerRot::YXZ, update_yaw, pitch, roll);//apply yaw change to hitbox
        //store camera changes to struct
        look_angle.pitch = delta_pitch;//pass delta and let camera apply pitch change so player does not tip over
        look_angle.yaw = update_yaw;
    }else{
        look_angle.pitch = 0.0;
    }
}