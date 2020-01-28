#![feature(test)]
#![feature(specialization)]

#[cfg(feature = "derive")]
pub use denc_derive::*;

use denc::*;

extern crate test;
use test::Bencher;

use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;
use rand::SeedableRng;

#[derive(Default, MapperDec)]
pub struct TestStruct {
    pub a: u16,
    pub b: u32,
}

#[derive(Default, MapperDec)]
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

#[derive(Default, MapperDec)]
pub struct TestStructArray {
    pub a87: u16,
    pub a32: u32,
    pub a35: u32,
    pub a23: u16,
    pub a42: u8,
    pub a41: u8,
    pub a53: [u16; 10],
    pub a7: u8,
    pub a47: u8,
}

#[derive(Default, MapperDec)]
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

#[derive(Default, MapperDec)]
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

fn main() {
    let mut small_rng = SmallRng::from_entropy();
    (0..5).for_each(|_| {
        let mut bytes = vec![0u8; 100];
        bytes[0] = 10;
        for b in bytes.iter_mut().skip(8) {
            *b = small_rng.gen();
        }
        (0..10000000).for_each(|_| {
            test::black_box(&bytes);
            let mut bytes = LittleEndian(&bytes[..]);
            let pong: TestStructVec = bytes.decode().unwrap();
            test::black_box(pong);
        });
    });
    println!("done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}

    pub struct TestStructTiny {
        pub a: u16,
        pub b: u8,
    }

    impl<Dec: Decoder> Decode<Dec> for TestStructTiny {
        const SIZE: usize = <u16 as Decode<Dec>>::SIZE + <u8 as Decode<Dec>>::SIZE;

        fn decode(decoder: &mut Dec, value: &mut TestStructTiny) -> Result<(), Dec::Error> {
            <u16 as Decode<Dec>>::decode(decoder, &mut value.a)?;
            <u8 as Decode<Dec>>::decode(decoder, &mut value.b)?;
            Ok(())
        }
    }

    #[derive(MapperDec)]
    pub struct TestStructTinyDerive {
        pub a: u16,
        pub b: u8,
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
                let mut pong: TestStructSmall = bytes.decode().unwrap();
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
                let pong: TestStructLarge = bytes.decode().unwrap();
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
                let pong: TestStructArray = bytes.decode().unwrap();
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
                let pong: TestStructVec = bytes.decode().unwrap();
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
                let pong: TestStructSmall = bytes.decode().unwrap();
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
                let pong: TestStructLarge = bytes.decode().unwrap();
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
                let pong: TestStructArray = bytes.decode().unwrap();
                test::black_box(pong);
            });
        });
    }
}
