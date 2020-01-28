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

    fn decode<'a>(data: &'a mut T, value: &mut Self) -> Result<(), T::Error>;
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

    default fn decode<'a>(data: &'a mut Dec, value: &mut V) -> Result<(), Dec::Error> {
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
    pub fn decode<T: Decode<Self> + Default>(&mut self) -> Result<T, &'static str> {
        self.fill_buffer(T::SIZE)?;
        let mut r = Default::default();
        T::decode(self, &mut r)?;
        Ok(r)
    }

    #[inline(always)]
    fn buff_advance_exact_const<'b, const N: usize>(&'b mut self) -> Option<&'b [u8; N]> {
        let (next, new) = split_at_const::<N>(self.0)?;
        self.0 = new;
        return Some(next);
    }

    #[inline(always)]
    fn buff_advance_exact<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        if self.0.len() < len {
            return None;
        }
        let r = self.0.get(0..len)?;
        self.0 = self.0.get(len..)?;
        return Some(r);
    }
}

#[inline(always)]
fn split_at_const<const N: usize>(slice: &[u8]) -> Option<(&[u8; N], &[u8])> {
    // if slice.len() < N {
    //     return None;
    // }
    let r = slice.get(0..N)?;
    let r2 = slice.get(N..)?;
    let ptr = r.as_ptr();
    //cast *u8 to *[u8; N] this should be fine I think?
    let ptr: *const [u8; N] = ptr.cast();
    //dereference ptr
    let ptr = unsafe { &*ptr };
    Some((&ptr, r2))
}

#[inline(always)]
fn split_at(slice: &[u8], at: usize) -> Option<(&[u8], &[u8])> {
    // if slice.len() < at {
    //     return None;
    // }
    let r = slice.get(0..at)?;
    let r2 = slice.get(at..)?;
    Some((r, r2))
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
    fn decode<'b>(data: &'b mut LittleEndian<'a>, value: &mut u8) -> Result<(), &'static str> {
        match data.0 {
            &[x, ref inner @ ..] => {
                data.0 = inner;
                *value = x;
                Ok(())
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>, value: &mut u16) -> Result<(), &'static str> {
        match data.0 {
            &[x1, x2, ref inner @ ..] => {
                data.0 = inner;
                *value = u16::from_le_bytes([x1, x2]);
                Ok(())
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>, value: &mut u32) -> Result<(), &'static str> {
        //this is slower for some reason????
        // match data.0 {
        //     &[x1, x2, x3, x4, ref inner @ ..] => {
        //         data.0 = inner;
        //         Ok(u32::from_le_bytes([x1, x2, x3, x4]))
        //     }
        //     _ => Err(EOF),
        // }
        let slice = data.buff_advance_exact(4).ok_or(EOF)?;
        *value = u32::from_le_bytes(slice.try_into().ok().ok_or(EOF)?);
        Ok(())
    }
}

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 0;

    #[inline(always)]
    fn decode<'b>(
        data: &'b mut LittleEndian<'a>,
        value: &mut &'a [u8],
    ) -> Result<(), &'static str> {
        *value = &data.0.get(..).ok_or(EOF)?;
        Ok(())
    }
}

use std::mem::{self, MaybeUninit};
impl<'a, V: Decode<LittleEndian<'a>> + Default + Sized + Copy, const N: usize>
    Decode<LittleEndian<'a>> for [V; N]
{
    const SIZE: usize = V::SIZE * N;

    #[inline(always)]
    fn decode<'b>(
        data: &'b mut LittleEndian<'a>,
        value: &mut [V; { N }],
    ) -> Result<(), &'static str> {
        for elem in value.iter_mut() {
            if data.len() < V::SIZE {
                return Err(EOF);
            }
            V::decode(data, elem)?;
        }
        Ok(())
    }
}

impl<'a, V: Decode<LittleEndian<'a>> + Default> Decode<LittleEndian<'a>> for Vec<V> {
    const SIZE: usize = <u32 as Decode<LittleEndian<'a>>>::SIZE;
    //const STATIC: bool = false;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>, value: &mut Vec<V>) -> Result<(), &'static str> {
        let mut size = 0u32;
        <u32 as Decode<LittleEndian<'a>>>::decode(data, &mut size)?;
        if data.len() < V::SIZE * size as usize {
            return Err(EOF);
        }
        value.clear();
        value.reserve(size as usize);
        for _ in 0..size {
            let mut v = V::default();
            V::decode(data, &mut v)?;
            value.push(v);
        }
        //<&[u8;4]>
        Ok(())
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
    pub fn decode<T: Decode<Self> + Default>(&mut self) -> Result<T, &'static str> {
        self.fill_buffer(T::SIZE)?;
        let mut t = Default::default();
        T::decode(self, &mut t)?;
        Ok(t)
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
    fn decode<'b>(
        data: &'b mut LittleEndianReader<'a, R>,
        value: &mut u8,
    ) -> Result<(), &'static str> {
        *value = *data.buff_advance_exact(1).ok_or(EOF)?.get(0).ok_or(EOF)?;
        Ok(())
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(
        data: &'b mut LittleEndianReader<'a, R>,
        value: &mut u16,
    ) -> Result<(), &'static str> {
        let buff = data.buff_advance_exact(2).ok_or(EOF)?;
        *value = u16::from_le_bytes(buff.try_into().ok().ok_or(EOF)?);
        Ok(())
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(
        data: &'b mut LittleEndianReader<'a, R>,
        value: &mut u32,
    ) -> Result<(), &'static str> {
        let buff = data.buff_advance_exact(4).ok_or(EOF)?;
        *value = u32::from_le_bytes(buff.try_into().ok().ok_or(EOF)?);
        Ok(())
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
    fn decode<'b>(
        data: &'b mut LittleEndianReader<'a, R>,
        value: &mut [V; { N }],
    ) -> Result<(), &'static str> {
        for elem in value.iter_mut() {
            V::decode(data, elem)?;
        }
        Ok(())
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
