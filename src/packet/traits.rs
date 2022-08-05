use bytes::{Bytes, BytesMut};
use uuid::Uuid;

use super::header::{PacketHeader, PacketType};

pub trait PacketBytes {
    fn write_bytes(&self, bytes: &mut BytesMut) -> usize;
    fn from_bytes(bytes: &mut Bytes) -> Self;
}

pub trait Packet: PacketBytes + Into<PacketType> {
    #[inline]
    fn into_header(self, id: Uuid) -> PacketHeader {
        PacketHeader {
            id,
            packet: self.into(),
        }
    }
}
