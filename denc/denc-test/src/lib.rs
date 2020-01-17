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
