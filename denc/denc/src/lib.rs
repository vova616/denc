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

pub trait Encode<'b, T: Encoder<'b>> {
    #[inline(always)]
    fn encode(&self, encoder: &'b mut [T::Data]);

    #[inline(always)]
    fn size(&self) -> usize;
}

pub trait Decode<'b, T: Decoder<'b>> {
    #[inline(always)]
    fn decode(decoder: &'b [T::Data]) -> Self;

    #[inline(always)]
    fn size(decoder: &'b [T::Data]) -> usize;
}

impl<'b, T, Decr: Decoder<'b>> Decode<'b, Decr> for T {
    #[inline(always)]
    default fn decode(decoder: &'b [Decr::Data]) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    default fn size(decoder: &'b [Decr::Data]) -> usize {
        unimplemented!()
    }
}

impl<'b, T, Encr: Encoder<'b>> Encode<'b, Encr> for T {
    #[inline(always)]
    default fn encode(&self, encoder: &'b mut [Encr::Data]) {
        unimplemented!()
    }

    #[inline(always)]
    default fn size(&self) -> usize {
        unimplemented!()
    }
}

struct LittleEndian {}
struct BigEndian {}

impl<'b> Decoder<'b> for LittleEndian {
    type Data = u8;
}

impl<'b> Encoder<'b> for LittleEndian {
    type Data = u8;
}

impl<'b> Decode<'b, LittleEndian> for u8 {
    #[inline(always)]
    fn decode(decoder: &'b [u8]) -> Self {
        decoder[0]
    }

    #[inline(always)]
    fn size(decoder: &'b [u8]) -> usize {
        1
    }
}

impl<'b> Decode<'b, LittleEndian> for u16 {
    #[inline(always)]
    fn decode(decoder: &'b [u8]) -> Self {
        u16::from_le_bytes([decoder[0], decoder[1]])
    }

    #[inline(always)]
    fn size(decoder: &'b [u8]) -> usize {
        2
    }
}

impl<'b> Encode<'b, LittleEndian> for u8 {
    #[inline(always)]
    fn encode(&self, decoder: &'b mut [u8]) {
        decoder[0] = *self;
    }

    #[inline(always)]
    fn size(&self) -> usize {
        1
    }
}

impl<'b> Encode<'b, LittleEndian> for u16 {
    #[inline(always)]
    fn encode(&self, decoder: &'b mut [u8]) {
        let bytes = self.to_le_bytes();
        decoder[0] = bytes[0];
        decoder[1] = bytes[1];
    }

    #[inline(always)]
    fn size(&self) -> usize {
        2
    }
}

impl<'b> Decoder<'b> for BigEndian {
    type Data = u8;
}

impl<'b> Encoder<'b> for BigEndian {
    type Data = u8;
}

impl<'b> Decode<'b, BigEndian> for u8 {
    #[inline(always)]
    fn decode(decoder: &'b [u8]) -> Self {
        decoder[0]
    }

    #[inline(always)]
    fn size(decoder: &'b [u8]) -> usize {
        1
    }
}

impl<'b> Decode<'b, BigEndian> for u16 {
    #[inline(always)]
    fn decode(decoder: &'b [u8]) -> Self {
        u16::from_be_bytes([decoder[0], decoder[1]])
    }

    #[inline(always)]
    fn size(decoder: &'b [u8]) -> usize {
        2
    }
}

pub trait Decoder<'b> {
    type Data;
}

pub trait Encoder<'b> {
    type Data;
}

pub trait DecoderEx<'b, Data, Output> {
    fn decoder(decoder: &'b [Data]) -> Output;
    fn sizer(decoder: &'b [Data]) -> usize;
}

impl<'b, Data, Decr: Decoder<'b, Data = Data>, Output: Decode<'b, Self> + Sized>
    DecoderEx<'b, Data, Output> for Decr
{
    fn decoder(decoder: &'b [Data]) -> Output {
        Decode::<Decr>::decode(decoder)
    }

    fn sizer(decoder: &'b [Data]) -> usize {
        <Output as Decode<'b, Decr>>::size(decoder)
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

    impl<'b, Data, Decr: Decoder<'b, Data = Data>> Decode<'b, Decr> for TestStructTiny {
        #[inline(always)]
        fn decode(decoder: &'b [Data]) -> Self {
            let a_size = <u16 as Decode<Decr>>::size(decoder);
            let (target, decoder) = decoder.split_at(a_size);
            let a: u16 = <u16 as Decode<'b, Decr>>::decode(target);
            let b_size = <u8 as Decode<Decr>>::size(decoder);
            let (target, decoder) = decoder.split_at(b_size);
            let b: u8 = <u8 as Decode<'b, Decr>>::decode(target);
            TestStructTiny { a: a, b: b }
        }

        #[inline(always)]
        fn size(decoder: &'b [Data]) -> usize {
            3
        }
    }

    impl<'b, Data, Encdr: Encoder<'b, Data = Data>> Encode<'b, Encdr> for TestStructTiny {
        #[inline(always)]
        fn encode(&self, encoder: &'b mut [Data]) {
            let a_size = <u16 as Encode<'b, Encdr>>::size(&self.a);
            let (target, encoder) = encoder.split_at_mut(a_size);
            let a = <u16 as Encode<'b, Encdr>>::encode(&self.a, target);
            let b_size = <u8 as Encode<'b, Encdr>>::size(&self.b);
            let (target, encoder) = encoder.split_at_mut(b_size);
            let b = <u8 as Encode<'b, Encdr>>::encode(&self.b, target);
        }

        #[inline(always)]
        fn size(&self) -> usize {
            3
        }
    }

    #[derive(MapperDec, MapperEnc)]
    pub struct TestDerive {
        pub a: u16,
        pub b: u8,
    }

    #[test]
    fn test_decode_tiny() {
        let bytes = [1u8, 0, 2];
        let u: u8 = LittleEndian::decoder(&bytes[..1]);
        let u2: u16 = LittleEndian::decoder(&bytes[..2]);
        let u3: u8 = BigEndian::decoder(&bytes[..1]);
        let u4: u16 = BigEndian::decoder(&bytes[..2]);

        assert_eq!(
            <TestStructTiny as Decode<LittleEndian>>::size(&bytes[..]),
            3
        );
        let b: TestStructTiny = LittleEndian::decoder(&bytes[..]);
        assert_eq!(b.a, 1);
        assert_eq!(b.b, 2);

        assert_eq!(<TestDerive as Decode<LittleEndian>>::size(&bytes[..]), 3);
        let c: TestDerive = LittleEndian::decoder(&bytes[..]);
        assert_eq!(c.a, 1);
        assert_eq!(c.b, 2);
    }

    #[test]
    fn test_encode_tiny() {
        let t = TestStructTiny { a: 1, b: 2 };
        let buffer = &mut [0u8; 3];
        <TestStructTiny as Encode<LittleEndian>>::encode(&t, &mut buffer[..]);
        assert_eq!(buffer, &[1, 0, 2]);
    }
}
