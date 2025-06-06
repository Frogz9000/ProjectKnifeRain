use std::{net::UdpSocket, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody, Velocity};
use multiplayer_lib::UpdatePacket;

use crate::player::Player;

pub struct NetcodePlugin;

const HOST_ADDRESS: &str = "0.0.0.0:25565";
const OTHER_ADDRESS: &str = "0.0.0.0:25566";

impl Plugin for NetcodePlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(UdpSocketResource::default())
            .add_systems(Update, bind_socket)
            .add_systems(Startup, setup_other_player)
            .add_systems(FixedUpdate, (send_pos_vel, update_other_player)
                .run_if(on_timer(Duration::from_millis(100)))
            )
            ;
    }
}

#[derive(Resource, Default, Debug)]
pub struct UdpSocketResource(Option<UdpSocket>);

impl UdpSocketResource{
    fn get_mut(&mut self)->&mut Option<UdpSocket>{
        &mut self.0
    }
    fn get(&self)->&Option<UdpSocket>{
        &self.0
    }
}

fn bind_socket(mut r_socket: ResMut<UdpSocketResource>){
    let Some(socket) = UdpSocket::bind(HOST_ADDRESS).ok() else {return};
    if let Err(_) = socket.set_nonblocking(true) {
        if let Err(_) = socket.set_read_timeout(Some(Duration::from_nanos(1))) {return};
        if let Err(_) = socket.set_write_timeout(Some(Duration::from_nanos(1))) {return};
    }
    *r_socket.get_mut() = Some(socket);
}


#[derive(Component)]
struct OtherPlayer;
fn setup_other_player(
    mut commands: Commands
){
    commands.spawn((
        OtherPlayer,
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.3),//default player hitbox for now
        LockedAxes::ROTATION_LOCKED,//prevent physics induced rotation, manual rotation done from input
        Transform::from_xyz(10.0, 1.0, 0.0),
        Velocity::zero(),
        Visibility::default(),
    ));
}


fn send_pos_vel(
    mut player: Query<(&Transform, &Velocity), With<Player>>,
    socket: Res<UdpSocketResource>
){
    let Some(socket) = socket.get() else {return};
    let Ok((transform, vel)) = player.single_mut() else {return};

    let packet = UpdatePacket::new(
        transform.translation,
        vel.linvel
    );

    packet.send_udp_to(socket, OTHER_ADDRESS);
}

fn update_other_player(
    mut player: Query<(&mut Transform,&mut Velocity), With<OtherPlayer>>,
    socket_input: Res<UdpSocketResource>,
){
    let Some(socket) = socket_input.get() else {return};
    let Ok((mut transform, mut velocity)) = player.single_mut() else {return};

    let Some(packet) = UpdatePacket::read_udp(socket) else {return};

    velocity.linvel = packet.velocity();
    transform.translation = packet.position();
}


