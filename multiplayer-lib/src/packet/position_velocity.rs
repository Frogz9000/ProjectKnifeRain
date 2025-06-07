use bevy::math::Vec3;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::packet::NetObjectId;

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