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

trait Decode<T>: Decoder {
    fn decode<'a>(data: &'a mut Self) -> T;
    fn size(data: &Self) -> usize;
}

trait Decoder {
    fn advance(&mut self, len: usize);
}

impl<V, Dec: Decoder> Decode<V> for Dec {
    default fn decode<'a>(data: &'a mut Self) -> V {
        unimplemented!()
    }

    default fn size(_: &Self) -> usize {
        unimplemented!()
    }
}

struct LittleEndian<'a>(&'a [u8]);

impl<'a> Decoder for LittleEndian<'a> {
    fn advance(&mut self, len: usize) {
        self.0 = &self.0[len..];
    }
}

impl<'a> Decode<u8> for LittleEndian<'a> {
    fn decode<'b>(data: &'b mut Self) -> u8 {
        let r = data.0[0];
        data.advance(1);
        r
    }

    fn size(_: &Self) -> usize {
        1
    }
}

impl<'a> Decode<u16> for LittleEndian<'a> {
    fn decode<'b>(data: &'b mut Self) -> u16 {
        let r = u16::from_le_bytes([data.0[0], data.0[1]]);
        data.advance(2);
        r
    }

    fn size(_: &Self) -> usize {
        2
    }
}

impl<'a> Decode<&'a [u8]> for LittleEndian<'a> {
    fn decode<'b>(data: &'b mut Self) -> &'a [u8] {
        let (x, y) = data.0.split_at(1);
        data.0 = y;
        x
    }

    fn size(_: &Self) -> usize {
        1
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
        fn decode(mut decoder: &mut Dec) -> TestStructTiny {
            let a: u16 = <Dec as Decode<u16>>::decode(decoder);
            let b: u8 = <Dec as Decode<u8>>::decode(decoder);
            TestStructTiny { a: a, b: b }
        }

        fn size(_: &Dec) -> usize {
            3
        }
    }

    pub struct TestStructTinyRef<'a> {
        pub a: u16,
        pub b: u8,
        pub c: &'a [u8],
        pub e: &'a [u8],
    }

    impl<'a, Dec: Decoder> Decode<TestStructTinyRef<'a>> for Dec {
        fn decode(decoder: &mut Dec) -> TestStructTinyRef<'a> {
            let a: u16 = <Dec as Decode<u16>>::decode(decoder);
            let b: u8 = <Dec as Decode<u8>>::decode(decoder);
            let c: &'a [u8] = <Dec as Decode<&'a [u8]>>::decode(decoder);
            let e: &'a [u8] = <Dec as Decode<&'a [u8]>>::decode(decoder);
            TestStructTinyRef {
                a: a,
                b: b,
                c: c,
                e: e,
            }
        }

        fn size(_: &Dec) -> usize {
            3
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
    fn test_encode_tiny() {}
}
