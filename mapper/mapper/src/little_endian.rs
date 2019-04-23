use crate::{Decoder, Encoder};
use std::convert::TryInto;
use std::io::prelude::Write;

impl<'b> Decoder<'b> for u8 {
    type Output = u8;

    #[inline(always)]
    fn decode(buffer: &[u8]) -> u8 {
        buffer[0]
    }

    #[inline(always)]
    fn size(buffer: &[u8]) -> usize {
        1
    }
}

impl<'b> Decoder<'b> for u16 {
    type Output = u16;

    #[inline(always)]
    fn decode(buffer: &[u8]) -> u16 {
        u16::from_le_bytes(buffer.try_into().unwrap())
    }

    #[inline(always)]
    fn size(buffer: &[u8]) -> usize {
        2
    }
}


impl<'b> Decoder<'b> for u32 {
    type Output = u32;

    #[inline(always)]
    fn decode(buffer: &[u8]) -> u32 {
        Self::from_le_bytes(buffer.try_into().unwrap())
    }

    #[inline(always)]
    fn size(buffer: &[u8]) -> usize {
        4
    }
}

impl Encoder for u8 {

    #[inline(always)]
    fn encode_into<T : Write>(&self, buff: &mut T) {
        let arr = self.to_le_bytes();
        buff.write(&arr[..]).unwrap();
    }

    #[inline(always)]
    fn size_enc(&self) -> usize {
        1
    }
}

impl Encoder for u16 {

    #[inline(always)]
    fn encode_into<T : Write>(&self, buff: &mut T) {
        let arr = self.to_le_bytes();
        buff.write(&arr[..]).unwrap();
    }

    #[inline(always)]
    fn size_enc(&self) -> usize {
        2
    }
}

impl Encoder for u32 {

    #[inline(always)]
    fn encode_into<T : Write>(&self, buff: &mut T) {
        let arr = self.to_le_bytes();
        buff.write(&arr[..]).unwrap();
    }

    #[inline(always)]
    fn size_enc(&self) -> usize {
        4
    }
}