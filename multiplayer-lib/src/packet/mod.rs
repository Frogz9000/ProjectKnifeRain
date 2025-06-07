pub mod position_velocity;

use std::net::{ToSocketAddrs, UdpSocket};
use bincode::{config::Configuration, Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::packet::position_velocity::PositionVelocity;

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
    Acknowledgement(PacketId),
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
    pub fn requires_acknowledgement(&self)->bool{
        self.packet_data.requires_acknowledgement()
    }
}
impl PacketData{
    /// If a packet requires acknowledgement,
    /// then the packet should be sent over and over until an acknowledgement is received.
    /// 
    /// Acknowledgement cant require acknowledgement or else there will be an infinite loop
    /// 
    /// Instantanious events (as opposed to continous sent data) should require acknowledgements
    pub fn requires_acknowledgement(&self)->bool{
        match self {
            PacketData::Acknowledgement(_) => false,
            PacketData::SpawnNetObject(_) => true,
            PacketData::DespawnNetObject(_) => true,
            PacketData::PositionVelocity(_) => false,
        }
    }
}