use std::f32::consts::FRAC_PI_2;

use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody, Velocity};
use bevy::{
    input::mouse::AccumulatedMouseMotion, prelude::*, render::view::RenderLayers
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,(
            setup_player,
        ));
        app.add_systems(Update,(
            update_player_keyboard_event,
            update_player_mouse_event,
            update_pov,
        ));
    }
}
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Speed(pub f32);
#[derive(Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);
impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))//arbitrary value, add settings controller later
    }
}
#[derive(Component)]
struct CameraController;
#[derive(Component)]
struct ViewModelCamera;
#[derive(Component)]
struct WorldCamera;
#[derive(Component)]
struct CameraPitch(f32);//store camera pitch so player hitbox does not move on that axis

const VIEWMODEL_RENDER_LAYER: usize = 1;
const CAMERA_OFFSET_Z: f32 = 0.0;//apply to camera to lag behind hitbox for debug, set to 0 for first person
const CAMERA_OFFSET_Y: f32 = 0.5;//height offset to have camera at a certain level of player hitbox, not bottom of hitbox

fn setup_player(
    mut commands: Commands
){
    commands.spawn((
        Player,
        Speed(5.0),//adjust as needed for base speed, maybe have items or other modify later: 100 ~ 1m/s
        CameraSensitivity::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.3),//default player hitbox for now
        LockedAxes::ROTATION_LOCKED,//prevent physics induced rotation, manual rotation done from input
        Transform::from_xyz(0.0, 1.0, 0.0),
        Velocity::zero(),
        Visibility::default(),
    )).with_children(|player| {
        player.spawn((
            CameraController,
            CameraPitch(0.0),
            Transform::from_xyz(0.0,CAMERA_OFFSET_Y,CAMERA_OFFSET_Z),//child transforms are relative to parent
            Visibility::default(),
        )).with_children(|controller|{
            controller.spawn((
            WorldCamera,
            Camera3d::default(),
            Projection::from(PerspectiveProjection{
            fov: 90.0_f32.to_radians(),
            ..default()
            }),
        ));
            //spawn view model camera as child: immut fov 70 may change to depending on view model generated
            controller.spawn((
            ViewModelCamera,
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
        //Add parent.spawn viewmodel mesh when ready
        });
    });
}
fn update_player_keyboard_event(
    mut player: Query<(&Speed, &Transform,&mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
){
    let Ok((speed, transform, mut velocity)) = player.single_mut() else {return};
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW){
        direction += *transform.forward();
    }
    if input.pressed(KeyCode::KeyS){
        direction += *transform.back();
    }
    if input.pressed(KeyCode::KeyA){
        direction += *transform.left();
    }
    if input.pressed(KeyCode::KeyD){
        direction += *transform.right();
    }
    //flatten vector (ignore y)
    direction.y = 0.0;
    let direction = direction.normalize_or_zero();
    let mut current_speed = speed.0;
    //check for sprint
    if input.pressed(KeyCode::ShiftLeft){
        current_speed = speed.0 * 2.0;//for now double speed when sprinting consider changing to var that can change from gear
    }
    velocity.linvel = Vec3::new(
        direction.x * current_speed,
        velocity.linvel.y,
        direction.z * current_speed,
    )
}

fn update_player_mouse_event(
    accum_mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: ParamSet<(
        Query<(&mut Transform, &CameraSensitivity), With<Player>>,
        Query<(&mut Transform, &mut CameraPitch), With<CameraController>>,
    )>,
){
    let delta = accum_mouse_motion.delta;
    if delta == Vec2::ZERO {return}

    let mut bind1 = query.p0();
    let Ok((mut transform,camera_sensitivity)) = bind1.single_mut() else {return};
    let delta_yaw = -delta.x * camera_sensitivity.x;
    let delta_pitch = -delta.y * camera_sensitivity.y;
    let (yaw,_,_) = transform.rotation.to_euler(EulerRot::YXZ);
    //apply yaw change to hitbox
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw+delta_yaw, 0.0, 0.0);

    
    let mut bind2 = query.p1();
    let Ok((mut cam_trans, mut camera_pitch)) = bind2.single_mut() else{return};
    //update pitch for camera only to prevent strange physics interactions
    //prevent camera from going fully up or down to prevent ambiguity of what forward is and then reversing yaw
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    camera_pitch.0 = (camera_pitch.0 + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    //apply pitch changes to camera
    cam_trans.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, camera_pitch.0, 0.0);//this transform is relative to parent
}

//for now FOV will be controlled with up/down arrow keys for development -> Plan to move to settings.rs when made
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