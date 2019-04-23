
use mapper::{Decoder,Encoder, MapperDec, MapperEnc};

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

pub mod Send {
    use mapper::{Decoder,Encoder, MapperDec, MapperEnc};

    #[derive(MapperEnc)]
    pub struct Hello {
        pub payload: u32
    }
}


#[derive(Debug)]
pub struct Ping {
    payload: u16
}

pub enum Packet {
    Pong(Pong),
    Unknown{ id: u32 }
}

