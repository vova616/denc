#![feature(test)]
#![feature(specialization)]

#[cfg(feature = "derive")]
pub use denc_derive::*;

use denc::*;

#[derive(MapperDec)]
pub struct TestStruct {
    pub a: u16,
    pub b: u32,
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

#[derive(MapperDec)]
pub struct TestStructArray {
    pub a87: u16,
    pub a32: u32,
    pub a35: u32,
    pub a23: u16,
    pub a42: u8,
    pub a41: u8,
    pub a53: [u32; 10],
    pub a7: u8,
    pub a47: u8,
}

#[derive(MapperDec)]
pub struct TestStructVec {
    pub a53: Vec<u32>,
    pub a87: u16,
    pub a32: u32,
    pub a35: u32,
    pub a23: u16,
    pub a42: u8,
    pub a41: u8,
    pub a7: u8,
    pub a47: u8,
}

#[derive(MapperDec)]
pub struct TestStructLarge {
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
    pub a34: u32,
    pub a3: u32,
    pub a04: u8,
    pub a65: u32,
    pub a75: u8,
    pub a66: u32,
    pub a17: u8,
    pub a61: u32,
    pub a27: u16,
    pub a13: u8,
    pub a51: u16,
    pub a63: u32,
    pub a92: u32,
    pub a05: u8,
    pub a9: u32,
    pub a14: u8,
    pub a31: u32,
    pub a73: u8,
    pub a6: u32,
    pub a76: u8,
    pub a64: u32,
    pub a97: u32,
    pub a03: u8,
    pub a74: u8,
    pub a46: u8,
    pub a16: u8,
    pub a22: u16,
    pub a21: u16,
    pub a84: u16,
    pub a86: u16,
    pub a95: u32,
    pub a81: u16,
    pub a2: u16,
    pub a44: u8,
    pub a1: u8,
    pub a83: u16,
    pub a96: u32,
    pub a12: u8,
    pub a5: u16,
    pub a55: u16,
    pub a8: u16,
    pub a33: u32,
    pub a15: u8,
    pub a45: u8,
    pub a54: u16,
    pub a77: u8,
    pub a36: u32,
    pub a0: u8,
    pub a07: u8,
    pub a85: u16,
    pub a72: u8,
    pub a4: u8,
    pub a67: u32,
    pub a56: u16,
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

            decoder.fill_buffer(const_size);
            let a: u16 = <u16 as Decode<Dec>>::decode(decoder);
            const_size -= <u16 as Decode<Dec>>::SIZE;

            decoder.fill_buffer(const_size);
            let b: u8 = <u8 as Decode<Dec>>::decode(decoder);
            const_size -= <u8 as Decode<Dec>>::SIZE;

            decoder.fill_buffer(const_size);
            let c: &'a [u8] = <&'a [u8] as Decode<Dec>>::decode(decoder);
            const_size -= <&'a [u8] as Decode<Dec>>::SIZE;

            decoder.fill_buffer(const_size);
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
        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: u8 = Decode::decode(&mut bytes);

        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: TestStructTiny = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2, 1, 3] as &[u8]);
        let a: TestStructTinyRef = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
        assert_eq!(a.c, &[1u8]);
        assert_eq!(a.e, &[3u8]);
    }

    #[test]
    fn test_encode_tiny_derive() {
        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: TestStructTiny = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: TestStructTinyDerive = Decode::decode(&mut bytes);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
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
                let mut pong: TestStructSmall = bytes.decode();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_large(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1024];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndian(&bytes[..]);
                let mut pong: TestStructLarge = bytes.decode();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_array(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndian(&bytes[..]);
                let mut pong: TestStructArray = bytes.decode();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_vec(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            bytes[0] = 10;
            for b in bytes.iter_mut().skip(8) {
                *b = small_rng.gen();
            }
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndian(&bytes[..]);
                let mut pong: TestStructVec = bytes.decode();
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
                let mut pong: TestStructSmall = bytes.decode();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_large2(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1024];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let bytes = &bytes[..] as &[u8];
            let mut buffer = [0u8; 1024];
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndianReader::new(&bytes[..], &mut buffer[..1024]);
                let mut pong: TestStructLarge = bytes.decode();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_array2(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1024];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let bytes = &bytes[..] as &[u8];
            let mut buffer = [0u8; 1024];
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndianReader::new(&bytes[..], &mut buffer[..1024]);
                let mut pong: TestStructArray = bytes.decode();
                test::black_box(pong);
            });
        });
    }
}
