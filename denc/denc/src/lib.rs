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
    fn size(data: &T) -> usize;
}

pub trait Decoder {
    fn ensure(&mut self, len: usize);
}

impl<V, Dec: Decoder> Decode<Dec> for V {
    default const SIZE: usize = 0;

    default fn decode<'a>(data: &'a mut Dec) -> V {
        unimplemented!()
    }

    #[inline(always)]
    default fn size(_: &Dec) -> usize {
        <V as Decode<Dec>>::SIZE
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

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> &'a [u8] {
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

    #[test]
    fn test() {}

    pub struct TestStructTiny {
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

    impl<'a, Dec: Decoder> Decode<Dec> for TestStructTinyRef<'a> {
        const SIZE: usize = <u16 as Decode<Dec>>::SIZE
            + <u8 as Decode<Dec>>::SIZE
            + <&'a [u8] as Decode<Dec>>::SIZE
            + <&'a [u8] as Decode<Dec>>::SIZE;

        fn decode(decoder: &mut Dec) -> TestStructTinyRef<'a> {
            let mut const_size = <u16 as Decode<Dec>>::SIZE
                + <u8 as Decode<Dec>>::SIZE
                + <&'a [u8] as Decode<Dec>>::SIZE
                + <&'a [u8] as Decode<Dec>>::SIZE;

            decoder.ensure(const_size);
            decoder.ensure(<u16 as Decode<Dec>>::size(decoder));
            let a: u16 = <u16 as Decode<Dec>>::decode(decoder);
            const_size -= <u16 as Decode<Dec>>::SIZE;

            decoder.ensure(const_size);
            decoder.ensure(<u8 as Decode<Dec>>::size(decoder));
            let b: u8 = <u8 as Decode<Dec>>::decode(decoder);
            const_size -= <u8 as Decode<Dec>>::SIZE;

            decoder.ensure(const_size);
            decoder.ensure(<&'a [u8] as Decode<Dec>>::size(decoder));
            let c: &'a [u8] = <&'a [u8] as Decode<Dec>>::decode(decoder);
            const_size -= <&'a [u8] as Decode<Dec>>::SIZE;

            decoder.ensure(const_size);
            decoder.ensure(<&'a [u8] as Decode<Dec>>::size(decoder));
            let e: &'a [u8] = <&'a [u8] as Decode<Dec>>::decode(decoder);
            const_size -= <&'a [u8] as Decode<Dec>>::SIZE;

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
        let a: u8 = Decode::decode(&mut bytes);

        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: TestStructTiny = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2, 1, 3]);
        let a: TestStructTinyRef = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
        assert_eq!(a.c, &[1u8]);
        assert_eq!(a.e, &[3u8]);
    }

    #[test]
    fn test_encode_tiny_derive() {
        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: TestStructTiny = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2]);
        let a: TestStructTinyDerive = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
    }
}
