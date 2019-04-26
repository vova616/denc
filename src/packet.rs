
use mapper::{Decoder,Encoder, MapperDec, MapperEnc};
use bytes::{Bytes, BytesMut, Buf, BufMut, IntoBuf};

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

pub trait ClientHandler {

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

pub mod recv {
    use mapper::{Decoder,Encoder, MapperDec, MapperEnc};
    use crate::packet::PacketID;

    #[derive(MapperDec)]
    pub struct Hello {
        pub payload: u8
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

    #[inline]
    fn from(packet: Packet<T>) -> Self {
        let bytes = packet.encode();
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
