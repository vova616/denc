#![feature(test)]
#![feature(specialization)]

#[cfg(feature = "derive")]
pub use denc_derive::*;

use denc::*;

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
