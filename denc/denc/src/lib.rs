#![feature(associated_type_defaults)]
#![feature(generators, generator_trait)]
#![feature(const_generics)]
#![feature(test)]
#![feature(specialization)]

use smallvec::{smallvec, SmallVec};

use std::convert::{TryFrom, TryInto};
use std::io::prelude::{Read, Write};

#[cfg(feature = "derive")]
pub use denc_derive::*;

/**
One Way:
    size is a size hint
    is should not be too big or the reading wont strat
    before each decoding the size hint ensures that there are enough bytes to
    read the stream

    to decode can read as many bytes he wants

    Why do we need advance:
        I'm not sure.
        maybe for visitors but visitors can implement their own

        Cons: bad impl of advance is bad for visitors.

    Problem with size hint:
        we cannot get final size.
        but we dont need final size when decoding so its fine


*/

pub trait Decode<T: Decoder> {
    const SIZE: usize;

    fn decode<'a>(data: &'a mut T) -> Self;
}

pub trait Decoder {
    fn fill_buffer(&mut self, len: usize);
}

impl<V, Dec: Decoder> Decode<Dec> for V {
    default const SIZE: usize = 0;

    default fn decode<'a>(data: &'a mut Dec) -> V {
        unimplemented!()
    }
}

pub struct LittleEndian<'a>(pub &'a [u8]);

impl<'a> LittleEndian<'a> {
    #[inline(always)]
    fn advance(&mut self, len: usize) {
        self.0 = &self.0[len..];
    }
}

impl<'a> Decoder for LittleEndian<'a> {
    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) {
        assert!(self.0.len() >= len)
    }
}

impl<'a> Decode<LittleEndian<'a>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> u8 {
        let r = data.0[0];
        data.advance(1);
        r
    }
}

impl<'a> Decode<LittleEndian<'a>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> u16 {
        let r = u16::from_le_bytes([data.0[0], data.0[1]]);
        data.advance(2);
        r
    }
}

impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> u32 {
        let r = u32::from_le_bytes([data.0[0], data.0[1], data.0[2], data.0[3]]);
        data.advance(4);
        r
    }
}

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> &'a [u8] {
        let (x, y) = data.0.split_at(1);
        data.0 = y;
        x
    }
}

use std::ops::Range;

pub struct LittleEndianReader<'a, R: Read> {
    pub reader: R,
    pub buffer: &'a mut [u8],
    pub cursor: Range<usize>,
}

impl<'a, R: Read> LittleEndianReader<'a, R> {
    pub fn new(reader: R, buffer: &'a mut [u8]) -> Self {
        LittleEndianReader {
            reader,
            buffer,
            cursor: 0..0,
        }
    }

    fn inner(self) -> R {
        self.reader
    }

    fn buff<'b>(&'b self) -> &'b [u8] {
        &self.buffer[self.cursor.clone()]
    }

    /*
        Next Refactor:
        Remove fill_buffer from each line?
    */

    #[inline(always)]
    fn buff_advance<'b>(&'b mut self, len: usize) -> &'b [u8] {
        while self.cursor.len() < len {
            if self.buffer.len() < len + self.cursor.start {
                assert!(self.buffer.len() >= len);
                self.buffer.copy_within(self.cursor.clone(), 0);
                self.cursor = 0..self.cursor.len();
            }
            self.cursor.end += match self.reader.read(&mut self.buffer[self.cursor.end..]) {
                Ok(n) => n,
                Err(e) => panic!(e),
            };
            }
        assert!(self.cursor.len() >= len);
        let buff = &self.buffer[self.cursor.clone()];
        self.cursor.start += len;
        buff
    }
}

impl<'a, R: Read> Decoder for LittleEndianReader<'a, R> {
    #[inline(always)]
    fn fill_buffer(&mut self, len_hint: usize) {
        while self.cursor.len() < len_hint {
            if self.buffer.len() < len_hint + self.cursor.start {
                assert!(self.buffer.len() >= len_hint);
                self.buffer.copy_within(self.cursor.clone(), 0);
                self.cursor = 0..self.cursor.len();
            }
            self.cursor.end += match self.reader.read(&mut self.buffer[self.cursor.end..]) {
                Ok(n) => n,
                Err(e) => panic!(e),
            };
        }
        assert!(self.cursor.len() >= len_hint);
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> u8 {
        data.buff_advance(1)[0]
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> u16 {
        let buff = data.buff_advance(2);
        let r = u16::from_le_bytes([buff[0], buff[1]]);
        r
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> u32 {
        let buff = data.buff_advance(4);
        let r = u32::from_le_bytes([buff[0], buff[1], buff[2], buff[3]]);
        r
    }
}

/*
impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> &'a [u8] {
        let (x, y) = data.0.split_at(1);
        data.0 = y;
        x
    }
}*/

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;

    #[test]
    fn test() {
        let bytes = &[0u8, 1, 2, 3] as &[u8];
        read(&bytes);
    }

    pub struct TestStructTiny {
        pub a: u16,
        pub b: u8,
    }

    #[derive(MapperDec)]
    pub struct TestStructTinyDerive {
        pub a: u16,
        pub b: u8,
    }

    impl<Dec: Decoder> Decode<Dec> for TestStructTiny {
        const SIZE: usize = <u16 as Decode<Dec>>::SIZE + <u8 as Decode<Dec>>::SIZE;

        fn decode(decoder: &mut Dec) -> TestStructTiny {
            fill_buffer_smart::<Dec, { <u16 as Decode<Dec>>::DYNAMIC }>(decoder, 10);
            let a: u16 = <u16 as Decode<Dec>>::decode(decoder);
            let b: u8 = <u8 as Decode<Dec>>::decode(decoder);
            TestStructTiny { a: a, b: b }
        }
    }

    pub struct TestStruct {
        pub a: u16,
        pub b: u32,
    }
    impl<Dec: Decoder> Decode<Dec> for TestStruct {
        const SIZE: usize = <u16 as Decode<Dec>>::SIZE + <u32 as Decode<Dec>>::SIZE;
        #[inline]
        fn decode(decoder: &mut Dec) -> TestStruct {
            let _dec_a: usize = <u16 as Decode<Dec>>::SIZE + <u32 as Decode<Dec>>::SIZE;
            let _dec_b: usize = <u32 as Decode<Dec>>::SIZE;
            fill_buffer_smart::<Dec, { true }>(decoder, _dec_a);
            let a = <u16 as Decode<Dec>>::decode(decoder);
            fill_buffer_smart::<Dec, { <u32 as Decode<Dec>>::DYNAMIC }>(decoder, _dec_b);
            let b = <u32 as Decode<Dec>>::decode(decoder);
            TestStruct { a, b }
        }
    }

    fn read<R: Read>(reader: &R) {}
}
