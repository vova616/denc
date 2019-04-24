
use mapper::{Decoder,Encoder, MapperDec, MapperEnc};
use bytes::{Bytes, BytesMut, Buf, BufMut, IntoBuf};

#[derive(Debug)]
pub struct Pong {
    pub payload: u8
}

#[derive(MapperDec, MapperEnc)]
pub struct Rora {
   pub payload: u8,
   pub payload2: u16,
   pub payloadList: mapper::List<u16, u8>,
   pub payloadList2: [mapper::List<u8, u8>; 2]
}

#[derive(MapperDec, MapperEnc)]
pub struct Header {
    pub size: u16,
    pub encryption: u16,
    pub id: u32
}

impl Header {
    pub fn new(id: u32) -> Header {
        Header{size: 0, encryption: 1, id: id}
    }

    pub fn new_raw(id: u32) -> Header {
        Header{size: 0, encryption: 0, id: id}
    }
}

pub trait PacketID {
    const ID: u32;
}

pub mod send {
    use mapper::{Decoder,Encoder, MapperDec, MapperEnc};
    use crate::packet::PacketID;

    #[derive(MapperEnc)]
    pub struct Hello {
        pub payload: u32
    }

    impl Hello {
        pub fn new() -> Hello {
            Hello{payload: 0}
        }
    }

    impl PacketID for Hello {
        const ID: u32 = 0x66;
    }
}

#[derive(Debug)]
pub struct Ping {
    payload: u16
}

pub struct Packet<T> {
    pub header: Header,
    pub data: T,
}

impl<T : PacketID> Packet<T> {
    pub fn new(data: T) -> Packet<T> {
        Packet{header: Header::new(T::ID), data: data}
    }

    pub fn new_raw(data: T) -> Packet<T> {
        Packet{header: Header::new_raw(T::ID), data: data}
    }

}

impl<T: Encoder + PacketID> From<Packet<T>> for BytesMut  {
    fn from(packet: Packet<T>) -> Self {
        let mut bytes = packet.encode();
        BytesMut::from(&bytes[..])
    }
}

impl<'b, T: Decoder<'b,Output=T> + PacketID> Decoder<'b> for Packet<T> {
    type Output = Packet<T>;

    #[inline(always)]
    fn decode(buffer: &'b [u8]) -> Packet<T> {
        Packet{
            header: Header::decode(buffer),
            data: T::decode(buffer)
        }
    }

    #[inline(always)]
    fn size(buffer: &'b [u8]) -> usize {
        Header::size(buffer) + T::size(buffer)
    }
}

impl<T : Encoder  + PacketID> Encoder for  Packet<T> {
    #[inline(always)]
    fn encode_into<W : std::io::Write>(&self, buff: &mut W) {
        self.header.encode_into(buff);
        self.data.encode_into(buff);
    }
    #[inline(always)]
    fn size_enc(&self) -> usize {
        self.header.size_enc() + self.data.size_enc()
    }
}
