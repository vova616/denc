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
    const STATIC: bool;

    fn decode<'a>(data: &'a mut T) -> Self;
}

pub trait Decoder {
    fn fill_buffer(&mut self, len: usize);
    fn len(&self) -> usize;
}

impl<V, Dec: Decoder> Decode<Dec> for V {
    default const SIZE: usize = 0;
    default const STATIC: bool = false;

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

    #[inline(always)]
    pub fn decode<T: Decode<Self>>(&mut self) -> T {
        self.fill_buffer(T::SIZE);
        T::decode(self)
    }
}

impl<'a> Decoder for LittleEndian<'a> {
    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) {
        assert!(self.0.len() >= len)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
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
        self.fill_buffer(len);
        let buff = &self.buffer[self.cursor.clone()];
        self.cursor.start += len;
        buff
    }

    #[inline]
    pub fn decode<T: Decode<Self>>(&mut self) -> T {
        self.fill_buffer(T::SIZE);
        T::decode(self)
    }

    #[inline]
    pub fn fill_buffer_inner(&mut self, len: usize) {
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
}

impl<'a, R: Read> Decoder for LittleEndianReader<'a, R> {
    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) {
        while self.cursor.len() < len {
            self.fill_buffer_inner(len);
        }
        //assert!(self.cursor.len() >= len);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.cursor.len()
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
    use rand::rngs::SmallRng;
    use rand::FromEntropy;
    use rand::Rng;
    use rand::SeedableRng;

    #[test]
    fn test() {
        let bytes = &[0u8, 1, 2, 3] as &[u8];
    }

    pub struct TestStructTiny {
        pub a: u16,
        pub b: u8,
    }

    #[derive(MapperDec)]
    pub struct TestStructSmall {
        pub a87: u16,
        pub a32: u32,
        pub a35: u32,
        pub a23: u16,
        pub a42: u8,
        pub a41: u8,
        pub a7: u8,
        pub a47: u8,
        pub a53: u16,
        pub a25: u16,
        pub a94: u32,
        pub a37: u32,
        pub a11: u8,
        pub a02: u8,
        pub a52: u16,
        pub a43: u8,
        pub a57: u16,
        pub a82: u16,
        pub a01: u8,
        pub a91: u32,
        pub a62: u32,
        pub a26: u16,
        pub a06: u8,
        pub a24: u16,
        pub a71: u8,
        pub a93: u32,
    }

    /*
    #[derive(MapperDec)]
    pub struct TestStructTinyDerive {
        pub a: u16,
        pub b: u8,
    }

    impl<Dec: Decoder> Decode<Dec> for TestStructTiny {
        const SIZE: usize = <u16 as Decode<Dec>>::SIZE + <u8 as Decode<Dec>>::SIZE;

        fn decode(decoder: &mut Dec) -> TestStructTiny {
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
            let a = <u16 as Decode<Dec>>::decode(decoder);
            let b = <u32 as Decode<Dec>>::decode(decoder);
            TestStruct { a, b }
        }
    }

    #[bench]
    fn bench_decode_small(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndian(&bytes[..]);
                let mut pong: TestStructSmall = Decode::decode(&mut bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_small2(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let bytes = &bytes[..] as &[u8];
            let mut buffer = [0u8; 1024];
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndianReader::new(&bytes[..], &mut buffer[..]);
                let mut pong: TestStructSmall = Decode::decode(&mut bytes);
                test::black_box(pong);
            });
        });
    }

    fn read<R: Read>(reader: &R) {}
    */
}
