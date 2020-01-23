#![feature(associated_type_defaults)]
#![feature(generators, generator_trait)]
#![feature(const_generics)]
#![feature(test)]
#![feature(specialization)]
#![feature(const_if_match)]

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

pub trait Decode<T: Decoder>: Sized {
    const SIZE: usize;
    const STATIC: bool;

    fn decode<'a>(data: &'a mut T) -> Result<Self, T::Error>;
}

pub trait Decoder {
    type Error;
    const EOF: Self::Error;

    fn fill_buffer(&mut self, len: usize) -> Result<(), Self::Error>;
    fn len(&self) -> usize;
}

impl<V, Dec: Decoder> Decode<Dec> for V {
    default const SIZE: usize = 0;
    default const STATIC: bool = false;

    default fn decode<'a>(data: &'a mut Dec) -> Result<V, Dec::Error> {
        unimplemented!()
    }
}

pub struct LittleEndian<'a>(pub &'a [u8]);

const EOF: &'static str = "EOF";

impl<'a> LittleEndian<'a> {
    #[inline(always)]
    fn advance(&mut self, len: usize) -> Option<()> {
        self.0 = self.0.get(len..)?;
        Some(())
    }

    #[inline]
    pub fn decode<T: Decode<Self>>(&mut self) -> Result<T, &'static str> {
        self.fill_buffer(T::SIZE)?;
        T::decode(self)
    }

    #[inline(always)]
    fn buff_advance_exact<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        let r = self.0.get(0..len)?;
        self.0 = self.0.get(len..)?;
        Some(r)
    }
}

impl<'a> Decoder for LittleEndian<'a> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) -> Result<(), &'static str> {
        if self.0.len() < len {
            Err(EOF)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Decode<LittleEndian<'a>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<u8, &'static str> {
        match data.0 {
            &[x, ref inner @ ..] => {
                data.0 = inner;
                Ok(x)
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<u16, &'static str> {
        match data.0 {
            &[x1, x2, ref inner @ ..] => {
                data.0 = inner;
                Ok(u16::from_le_bytes([x1, x2]))
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<u32, &'static str> {
        match data.0 {
            &[x1, x2, x3, x4, ref inner @ ..] => {
                data.0 = inner;
                Ok(u32::from_le_bytes([x1, x2, x3, x4]))
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 0;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<&'a [u8], &'static str> {
        Ok(&data.0.get(..).ok_or(EOF)?)
    }
}

use std::mem::{self, MaybeUninit};
impl<'a, V: Decode<LittleEndian<'a>> + Default + Sized + Copy, const N: usize>
    Decode<LittleEndian<'a>> for [V; N]
{
    const SIZE: usize = V::SIZE * N;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<[V; { N }], &'static str> {
        let mut arr: [MaybeUninit<V>; { N }] = unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut arr[..N] {
            if data.len() < V::SIZE {
                return Err(EOF);
            }
            *elem = MaybeUninit::new(V::decode(data)?);
        }
        Ok(unsafe { *mem::transmute::<_, &[V; { N }]>(&arr) })
    }
}

impl<'a, V: Decode<LittleEndian<'a>>> Decode<LittleEndian<'a>> for Vec<V> {
    const SIZE: usize = <u32 as Decode<LittleEndian<'a>>>::SIZE;
    //const STATIC: bool = false;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<Vec<V>, &'static str> {
        let size: usize = <u32 as Decode<LittleEndian<'a>>>::decode(data)? as usize;
        let mut arr = Vec::<V>::with_capacity(size as usize);
        if data.len() < V::SIZE * size {
            return Err(EOF);
        }
        for _ in 0..size {
            arr.push(V::decode(data)?);
        }
        //<&[u8;4]>
        Ok(arr)
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
    fn buff_advance<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        self.fill_buffer(len).ok()?;
        let buff = self.buffer.get(self.cursor.clone())?;
        self.cursor.start += len;
        Some(buff)
    }

    #[inline(always)]
    fn buff_advance_exact<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        self.fill_buffer(len).ok()?;
        let buff = self
            .buffer
            .get(self.cursor.start..self.cursor.start + len)?;
        self.cursor.start += len;
        Some(buff)
    }

    #[inline]
    pub fn decode<T: Decode<Self>>(&mut self) -> Result<T, &'static str> {
        self.fill_buffer(T::SIZE)?;
        T::decode(self)
    }

    #[inline]
    pub fn fill_buffer_inner(&mut self, len: usize) -> Result<(), &'static str> {
        if self.buffer.len() < len + self.cursor.start {
            if self.buffer.len() < len {
                return Err("Buffer is too small");
            }
            self.buffer.copy_within(self.cursor.clone(), 0);
            self.cursor = 0..self.cursor.len();
        }
        self.cursor.end += match self.reader.read(&mut self.buffer[self.cursor.end..]) {
            Ok(n) => n,
            Err(e) => return Err("Read err"),
        };
        Ok(())
    }
}

impl<'a, R: Read> Decoder for LittleEndianReader<'a, R> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) -> Result<(), &'static str> {
        while self.cursor.len() < len {
            self.fill_buffer_inner(len)?;
        }
        Ok(())
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
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<u8, &'static str> {
        let r = data.buff_advance_exact(1).ok_or(EOF)?.get(0).ok_or(EOF)?;
        Ok(*r)
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<u16, &'static str> {
        let buff = data.buff_advance_exact(2).ok_or(EOF)?;
        let r = u16::from_le_bytes(buff.try_into().ok().ok_or(EOF)?);
        Ok(r)
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<u32, &'static str> {
        let buff = data.buff_advance_exact(4).ok_or(EOF)?;
        let r = u32::from_le_bytes(buff.try_into().ok().ok_or(EOF)?);
        Ok(r)
    }
}

impl<'a, R: Read, V: Decode<LittleEndianReader<'a, R>> + Copy, const N: usize>
    Decode<LittleEndianReader<'a, R>> for [V; N]
{
    const SIZE: usize = if V::SIZE * N > 1024 {
        V::SIZE
    } else {
        V::SIZE * N
    };
    const STATIC: bool = V::SIZE * N <= 1024;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<[V; { N }], &'static str> {
        let mut arr: [MaybeUninit<V>; { N }] = unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut arr[..] {
            *elem = MaybeUninit::new(V::decode(data)?);
        }
        Ok(unsafe { *mem::transmute::<_, &[V; { N }]>(&arr) })
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
