pub mod packet;
use std::collections::VecDeque;

pub use packet::Packet;
pub use packet::PacketData;
pub use packet::PacketId;


/// Assumes that each packet enqued has a unique Id
#[derive(Default)]
pub struct PacketQueue{
    queue: VecDeque<Packet>,
}
impl PacketQueue{
    pub fn enqueue(&mut self, packet: Packet){
        self.queue.push_back(packet);
    }
    pub fn pop(&mut self)->Option<Packet>{
        let packet = self.queue.pop_front();
        if let Some(packet) = &packet {
            if packet.requires_acknowledgement() {
                self.enqueue(packet.clone());
            }
        }
        packet
    }
    pub fn acknowledge(&mut self, id: PacketId){
        self.queue.retain(|p|p.id() == id);
    }
    pub fn len(&self)->usize{
        self.queue.len()
    }
}