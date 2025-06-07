use std::net::{ToSocketAddrs, UdpSocket};

use bincode::{config::Configuration, Decode, Encode};
use serde::{Deserialize, Serialize};
use bevy::math::Vec3;

const PACKET_SIZE: usize = 40;

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct Packet{
    id: PacketId,
    packet_data: PacketData,
}

pub type PacketId = usize;
pub type NetObjectId = u16;

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum PacketData{
    SpawnNetObject(NetObjectId),
    DespawnNetObject(NetObjectId),
    PositionVelocity(PositionVelocity),
}
impl Packet{
    pub fn new(time_stamp: PacketId, packet_data: PacketData)->Self{
        Self {id: time_stamp, packet_data }
    }
    pub fn id(&self)->PacketId{
        self.id
    }
    pub fn data(&self)->&PacketData{
        &self.packet_data
    }
    pub fn serialize(&self)->Option<[u8; PACKET_SIZE]>{
        let mut out_buf: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
        let Ok(_) = 
            bincode::encode_into_slice(self, &mut out_buf, bincode::config::standard())
            else {return None};
        return Some(out_buf);
    }
    pub fn deserialize(data: &[u8; PACKET_SIZE])->Option<Self>{
        let Ok((packet, _)) =
            bincode::decode_from_slice::<Self, Configuration>(data, bincode::config::standard())
            else {return None};
        return Some(packet);
    }
    pub fn read_udp(socket: &UdpSocket)->Option<Self>{
        let mut in_buf: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
        let Ok(_) = socket.recv_from(&mut in_buf) else {return None};
        Self::deserialize(&in_buf)
    }
    pub fn send_udp_to(&self, socket: &UdpSocket, address: impl ToSocketAddrs){
        let Some(out_buf) = self.serialize() else {return};
        let Ok(_) = socket.send_to(&out_buf, address) else {return};
    }
}


#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct PositionVelocity{
    object: NetObjectId,
    position: PacketVec3,
    velocity: PacketVec3
}
impl PositionVelocity{
    pub fn new(
        object: NetObjectId,
        position: impl Into<PacketVec3>,
        velocity: impl Into<PacketVec3>
    ) -> Self {
        Self{
            object,
            position: position.into(),
            velocity: velocity.into()
        }
    }
    pub fn velocity(&self)->Vec3{
        self.velocity.clone().into()
    }
    pub fn position(&self)->Vec3{
        self.position.clone().into()
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct PacketVec3(f32, f32, f32);
impl PacketVec3{
    pub fn new(x: f32, y: f32, z: f32)->Self{
        Self(x, y, z)
    }
}

impl From<PacketVec3> for Vec3{
    fn from(value: PacketVec3) -> Vec3 {
        Vec3::new(value.0, value.1, value.2)
    }
}
impl From<Vec3> for PacketVec3{
    fn from(value: Vec3) -> PacketVec3 {
        PacketVec3::new(value.x, value.y, value.z)
    }
}


