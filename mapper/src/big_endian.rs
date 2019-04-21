use crate::{Decoder};
use std::convert::TryInto;

impl<'b> Decoder<'b> for u8 {
    type Output = u8;

    fn decode(buffer: &[u8]) -> u8 {
        buffer[0]
    }

    fn size(buffer: &[u8]) -> usize {
        1
    }
}

impl<'b> Decoder<'b> for u16 {
    type Output = u16;

    fn decode(buffer: &[u8]) -> u16 {
        u16::from_be_bytes(buffer.try_into().unwrap())
    }

    fn size(buffer: &[u8]) -> usize {
        2
    }
}


impl<'b> Decoder<'b> for u32 {
    type Output = u32;

    fn decode(buffer: &[u8]) -> u32 {
        Self::from_be_bytes(buffer.try_into().unwrap())
    }

    fn size(buffer: &[u8]) -> usize {
        4
    }
}