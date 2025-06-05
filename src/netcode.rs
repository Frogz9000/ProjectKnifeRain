use std::net::UdpSocket;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody, Velocity};
use bincode::{config::Configuration, Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::player::Player;

pub struct NetcodePlugin;


impl Plugin for NetcodePlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(UdpSocketResource::default())
            .add_systems(Update, bind_socket)
            .add_systems(Startup, setup_other_player)
            .add_systems(Update, (send_my_data, update_other_player))
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
    let Some(socket) = UdpSocket::bind("0.0.0.0:25565").ok() else {return};
    // socket.connect("24.265.234.220:25565");
    *r_socket.get_mut() = Some(socket);
    println!("{:?}", r_socket);
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


fn send_my_data(
    mut player: Query<(&Transform, &Velocity), With<Player>>,
    socket: Res<UdpSocketResource>
){
    let Some(socket) = &socket.0 else {return};
    let Ok((transform, vel)) = player.single_mut() else {return};

    let packet = UpdatePacket::new(
        transform.translation,
        vel.linvel
    );

    let mut out_buf: [u8; 24] = [0; 24];
    let Ok(_) = bincode::encode_into_slice(packet, &mut out_buf, bincode::config::standard()) else {return};
    //"24.265.234.220:25565"
    let Ok(_) = socket.send_to(&out_buf, "0.0.0.0:25565") else {return};
}

fn update_other_player(
    mut player: Query<(&mut Transform,&mut Velocity), With<OtherPlayer>>,
    socket_input: Res<UdpSocketResource>,
    // input: Res<ButtonInput<KeyCode>>,
    // time: Res<Time>,
){
    let Some(socket) = socket_input.get() else {return};
    let mut in_buf: [u8; 24] = [0; 24];
    let Ok(_) = socket.recv_from(&mut in_buf) else {return};
    let Ok((packet, _)) =
        bincode::decode_from_slice::<UpdatePacket, Configuration>(&in_buf, bincode::config::standard()) else {return};

    let Ok((mut transform, mut velocity)) = player.single_mut() else {return};
    velocity.linvel = packet.vel();
    transform.translation = packet.pos() + Vec3::new(5.0, 0.0, 0.0);
}

#[derive(Serialize, Deserialize, Encode, Decode)]
struct UpdatePacket{
    position: (f32, f32, f32),
    velocity: (f32, f32, f32)
}
impl UpdatePacket{
    fn new(position: Vec3, velocity: Vec3)->Self{
        Self {
            position: (position.x, position.y, position.z),
            velocity: (velocity.x, velocity.y, velocity.z)
        }
    }
    fn vel(&self)->Vec3{
        Vec3::new(self.velocity.0, self.velocity.1, self.velocity.2)
    }
    fn pos(&self)->Vec3{
        Vec3::new(self.position.0, self.position.1, self.position.2)
    }
}
