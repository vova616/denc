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

pub trait Decode<T>: Decoder {
    const SIZE: usize;

    fn decode<'a>(data: &'a mut Self) -> T;
    fn size(data: &Self) -> usize;
}

pub trait Decoder {
    fn ensure(&mut self, len: usize);
}

impl<V, Dec: Decoder> Decode<V> for Dec {
    default const SIZE: usize = 0;

    default fn decode<'a>(data: &'a mut Self) -> V {
        unimplemented!()
    }

    #[inline(always)]
    default fn size(_: &Self) -> usize {
        <Self as Decode<V>>::SIZE
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
    fn ensure(&mut self, len: usize) {
        assert!(self.0.len() >= len);
    }
}

impl<'a> Decode<u8> for LittleEndian<'a> {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut Self) -> u8 {
        let r = data.0[0];
        data.advance(1);
        r
    }
}

impl<'a> Decode<u16> for LittleEndian<'a> {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut Self) -> u16 {
        let r = u16::from_le_bytes([data.0[0], data.0[1]]);
        data.advance(2);
        r
    }
}

impl<'a> Decode<&'a [u8]> for LittleEndian<'a> {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut Self) -> &'a [u8] {
        let (x, y) = data.0.split_at(1);
        data.0 = y;
        x
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;
    use rand::rngs::SmallRng;
    use rand::FromEntropy;
    use rand::Rng;
    use rand::SeedableRng;

    pub struct TestStructTiny {
        pub a: u16,
        pub b: u8,
    }

    impl<Dec: Decoder> Decode<TestStructTiny> for Dec {
        const SIZE: usize = <Dec as Decode<u16>>::SIZE + <Dec as Decode<u8>>::SIZE;

        fn decode(mut decoder: &mut Dec) -> TestStructTiny {
            let a: u16 = <Dec as Decode<u16>>::decode(decoder);
            let b: u8 = <Dec as Decode<u8>>::decode(decoder);
            TestStructTiny { a: a, b: b }
        }
    }

    #[derive(MapperDec)]
    pub struct TestStructTinyDerive {
        pub a: u16,
        pub b: u8,
    }

    pub struct TestStructTinyRef<'a> {
        pub a: u16,
        pub b: u8,
        pub c: &'a [u8],
        pub e: &'a [u8],
    }

    impl<'a, Dec: Decoder> Decode<TestStructTinyRef<'a>> for Dec {
        const SIZE: usize = <Dec as Decode<u16>>::SIZE
            + <Dec as Decode<u8>>::SIZE
            + <Dec as Decode<&'a [u8]>>::SIZE
            + <Dec as Decode<&'a [u8]>>::SIZE;

        fn decode(decoder: &mut Dec) -> TestStructTinyRef<'a> {
            let mut const_size = <Dec as Decode<u16>>::SIZE
                + <Dec as Decode<u8>>::SIZE
                + <Dec as Decode<&'a [u8]>>::SIZE
                + <Dec as Decode<&'a [u8]>>::SIZE;
            decoder.ensure(const_size);
            decoder.ensure(<Dec as Decode<u16>>::size(decoder));
            let a: u16 = <Dec as Decode<u16>>::decode(decoder);

            const_size -= <Dec as Decode<u16>>::SIZE;
            decoder.ensure(const_size);
            decoder.ensure(<Dec as Decode<u8>>::size(decoder));
            let b: u8 = <Dec as Decode<u8>>::decode(decoder);

            decoder.ensure(<Dec as Decode<&'a [u8]>>::SIZE + <Dec as Decode<&'a [u8]>>::SIZE);
            decoder.ensure(<Dec as Decode<&'a [u8]>>::size(decoder));
            let c: &'a [u8] = <Dec as Decode<&'a [u8]>>::decode(decoder);

            decoder.ensure(<Dec as Decode<&'a [u8]>>::SIZE);
            decoder.ensure(<Dec as Decode<&'a [u8]>>::size(decoder));
            let e: &'a [u8] = <Dec as Decode<&'a [u8]>>::decode(decoder);

            TestStructTinyRef {
                a: a,
                b: b,
                c: c,
                e: e,
            }
        }
    }

    #[test]
    fn test_decode_tiny() {
        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: u8 = LittleEndian::decode(&mut bytes);

        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: TestStructTiny = LittleEndian::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2, 1, 3]);
        let a: TestStructTinyRef = LittleEndian::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
        assert_eq!(a.c, &[1u8]);
        assert_eq!(a.e, &[3u8]);
    }

    #[test]
    fn test_encode_tiny_derive() {
        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: TestStructTiny = LittleEndian::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: TestStructTinyDerive = LittleEndian::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
    }
}
