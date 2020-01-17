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

trait Decoder<'a, T>: DecoderData {
    fn decode(data: &'a [Self::Data]) -> T;
    fn size(data: &'a [Self::Data]) -> usize;
}

trait DecoderData {
    type Data;
}

impl<'a, V, T: DecoderData> Decoder<'a, V> for T {
    default fn decode(data: &'a [Self::Data]) -> V {
        unimplemented!()
    }

    default fn size(_: &'a [Self::Data]) -> usize {
        unimplemented!()
    }
}

struct LittleEndian;

impl DecoderData for LittleEndian {
    type Data = u8;
}

impl<'a> Decoder<'a, u8> for LittleEndian {
    fn decode(data: &'a [Self::Data]) -> u8 {
        data[0]
    }

    fn size(_: &'a [Self::Data]) -> usize {
        1
    }
}

impl<'a> Decoder<'a, u16> for LittleEndian {
    fn decode(data: &'a [Self::Data]) -> u16 {
        u16::from_le_bytes([data[0], data[1]])
    }

    fn size(_: &'a [Self::Data]) -> usize {
        2
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

    impl<'a, T: DecoderData> Decoder<'a, TestStructTiny> for T {
        fn decode(data: &'a [Self::Data]) -> TestStructTiny {
            let a: u16 = <T as Decoder<'a, u16>>::decode(&data[..2]);
            let b: u8 = <T as Decoder<'a, u8>>::decode(&data[2..3]);
            TestStructTiny { a: a, b: b }
        }

        fn size(_: &'a [Self::Data]) -> usize {
            3
        }
    }

    #[test]
    fn test_decode_tiny() {
        let bytes = [1u8, 0, 2];
        let a: u8 = LittleEndian::decode(&bytes[..]);

        let a: TestStructTiny = LittleEndian::decode(&bytes[..]);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
    }

    #[test]
    fn test_encode_tiny() {}
}
