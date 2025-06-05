use std::net::UdpSocket;

use bevy::prelude::*;

pub struct NetcodePlugin;


impl Plugin for NetcodePlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(UdpSocketResource::default())
            .add_systems(Startup, bind_socket)
            ;
    }
}

#[derive(Resource, Default, Debug)]
pub struct UdpSocketResource(Option<UdpSocket>);

impl UdpSocketResource{
    fn get_mut(&mut self)->&mut Option<UdpSocket>{
        &mut self.0
    }
}

fn bind_socket(mut r_socket: ResMut<UdpSocketResource>){
    *r_socket.get_mut() = UdpSocket::bind("127.0.0.1:8000").ok();
    println!("{:?}", r_socket);
}

