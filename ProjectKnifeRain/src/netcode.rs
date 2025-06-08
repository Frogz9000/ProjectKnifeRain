use std::{net::UdpSocket, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rapier3d::prelude::{Collider, ExternalImpulse, LockedAxes, RigidBody, Velocity};
use multiplayer_lib::{packet::position_velocity::PositionVelocity, Packet, PacketData, PacketId, PacketQueue};

use crate::player::Player;

pub struct NetcodePlugin;
const NET_REFRESH_RATE_FPS: f32 = 30.0;

const HOST_ADDRESS: &str = "0.0.0.0:25566";
const OTHER_ADDRESS: &str = "0.0.0.0:25565";

impl Plugin for NetcodePlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(UdpSocketResource::default())
            .insert_resource(PacketIdFactory::default())
            .insert_resource(PacketQueueRes::default())
            .add_systems(Update, bind_socket)
            .add_systems(Startup, setup_other_player)
            .add_systems(FixedUpdate, (send_pos_vel, update_other_player, move_packet_queue)
                .run_if(on_timer(Duration::from_secs_f32(1.0 / NET_REFRESH_RATE_FPS)))
            )
            ;
    }
}

#[derive(Resource, Default, Debug)]
pub struct PacketIdFactory(PacketId);
impl PacketIdFactory{
    fn read(&mut self)->PacketId{
        self.0+=1;
        self.0-1
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

#[derive(Resource, Default)]
pub struct PacketQueueRes(PacketQueue);
fn move_packet_queue(
    mut r_packet_queue: ResMut<PacketQueueRes>,
    r_socket: Res<UdpSocketResource>
){
    let Some(socket) = r_socket.get() else {return};
    let mut i = r_packet_queue.0.len();
    while let Some(packet) = r_packet_queue.0.pop() {
        packet.send_udp_to(socket, OTHER_ADDRESS);
        i -= 1;
        if i == 0 {break}
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
        ExternalImpulse::default(),
    ));
}


fn send_pos_vel(
    mut player: Query<(&Transform, &Velocity), With<Player>>,
    mut time_stamp: ResMut<PacketIdFactory>,
    mut r_packet_queue: ResMut<PacketQueueRes>,
){
    let Ok((transform, vel)) = player.single_mut() else {return};

    let packet = Packet::new(
        time_stamp.read(),
        PacketData::PositionVelocity(
            PositionVelocity::new(
                0,
                transform.translation,
                vel.linvel
            )
        )
    );

    r_packet_queue.0.enqueue(packet);
}

fn update_other_player(
    mut player: Query<(&mut Transform,&mut Velocity), With<OtherPlayer>>,
    socket_input: Res<UdpSocketResource>,
    mut r_packet_queue: ResMut<PacketQueueRes>,
){
    let Some(socket) = socket_input.get() else {return};
    let Ok((mut transform, mut velocity)) = player.single_mut() else {return};

    let Some(packet) = Packet::read_udp(socket) else {return};

    match packet.data() {
        PacketData::PositionVelocity(position_velocity) => {
            velocity.linvel = position_velocity.velocity();
            transform.translation = position_velocity.position();
        },
        PacketData::Acknowledgement(id) => {
            r_packet_queue.0.acknowledge(*id);
        }
        _ => ()
    }
}