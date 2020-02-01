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

#[derive(Default, Denc, Enc)]
pub struct TestStruct {
    pub a: u16,
    pub b: u32,
}

#[derive(Default, Denc, Enc)]
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

#[derive(Default, Denc, Enc)]
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

#[derive(Default, Denc, Enc, Eq, PartialEq, Debug)]
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

#[derive(Default, Denc, Enc)]
pub struct TestStructLarge {
    pub a87: [TestStructSmall; 10],
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

#[derive(Default, Denc, Enc)]
pub struct TestStructTinyDerive {
    pub a: u16,
    pub b: u8,
}

#[derive(Default, Denc, Enc)]
pub struct TestStructTinyRef<'a> {
    pub a: u16,
    pub b: u8,
    pub c: &'a [u8],
    pub e: &'a [u8],
}

#[derive(Default, Denc, Enc)]
pub struct TestStructTinyT<T: Clone> {
    pub a: u16,
    pub b: T,
}

#[derive(Default, Denc, Enc)]
pub struct TestStructTiny {
    pub a: u16,
    pub b: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test() {}

    #[test]
    fn test_decode_tiny() {
        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: u8 = bytes.decode().unwrap();

        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: TestStructTiny = bytes.decode().unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2, 1, 3] as &[u8]);
        let a: TestStructTinyRef = bytes.decode().unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);
        assert_eq!(a.c, &[1u8, 3u8]);
    }

    #[test]
    fn test_encode_tiny_derive() {
        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: TestStructTiny = bytes.decode().unwrap();
        assert_eq!(a.a, 1);
        assert_eq!(a.b, 2);

        let mut bytes = LittleEndian(&[1u8, 0, 2] as &[u8]);
        let a: TestStructTinyDerive = bytes.decode().unwrap();
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
    fn bench_decode_vec_into(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            bytes[0] = 10;
            for b in bytes.iter_mut().skip(8) {
                *b = small_rng.gen();
            }
            let mut pong: TestStructVec = Default::default();
            pong.a53.reserve_exact(100);
            b.iter(|| {
                test::black_box(&bytes);
                let mut bytes = LittleEndian(&bytes[..]);
                bytes.decode_into(&mut pong).unwrap();
                test::black_box(&pong);
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
                let mut bytes = LittleEndianReader::new(&bytes[..]);
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
                let mut bytes = LittleEndianReader::new(&bytes[..]);
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
                let mut bytes: BufferedIO<_, _, 1024> = BufferedIO::new(&bytes[..]);
                let mut bytes = LittleEndianReader::new(bytes);
                let pong: TestStructArray = bytes.decode().unwrap();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_array3(b: &mut Bencher) {
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
                let mut bytes = LittleEndianReader::new(BufReader::new(bytes));
                let pong: TestStructArray = bytes.decode().unwrap();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_array4(b: &mut Bencher) {
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
                let mut bytes = LittleEndianReader::new(bytes);
                let pong: TestStructArray = bytes.decode().unwrap();
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_encode_small(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            bytes[0] = 10;
            for b in bytes.iter_mut().skip(8) {
                *b = small_rng.gen();
            }
            let pong: TestStructSmall = LittleEndian(&bytes[..]).decode().unwrap();
            b.iter(|| {
                test::black_box(&pong);
                let mut encoder = LittleEndianMut(&mut bytes[..]);
                encoder.encode_into(&pong).unwrap();
                test::black_box(&bytes[..]);
            });
        });
    }

    #[bench]
    fn bench_encode_large(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1024];
            bytes[0] = 10;
            for b in bytes.iter_mut().skip(8) {
                *b = small_rng.gen();
            }
            let pong: TestStructLarge = LittleEndian(&bytes[..]).decode().unwrap();
            b.iter(|| {
                test::black_box(&pong);
                let mut encoder = LittleEndianMut(&mut bytes[..]);
                encoder.encode_into(&pong).unwrap();
                test::black_box(&bytes[..]);
            });
        });
    }

    #[bench]
    fn bench_encode_array(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            bytes[0] = 10;
            for b in bytes.iter_mut().skip(8) {
                *b = small_rng.gen();
            }
            let pong: TestStructArray = LittleEndian(&bytes[..]).decode().unwrap();
            b.iter(|| {
                test::black_box(&pong);
                let mut encoder = LittleEndianMut(&mut bytes[..]);
                encoder.encode_into(&pong).unwrap();
                test::black_box(&bytes[..]);
            });
        });
    }

    #[bench]
    fn bench_encode_vec(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            bytes[0] = 10;
            for b in bytes.iter_mut().skip(8) {
                *b = small_rng.gen();
            }
            let pong: TestStructVec = LittleEndian(&bytes[..]).decode().unwrap();
            b.iter(|| {
                test::black_box(&pong);
                let mut encoder = LittleEndianMut(&mut bytes[..]);
                encoder.encode_into(&pong).unwrap();
                test::black_box(&bytes[..]);
            });
        });
    }

    #[test]
    fn test_encode_vec() {
        let mut small_rng = SmallRng::from_entropy();
        let mut bytes = vec![0u8; 100];
        bytes[0] = 10;
        for b in bytes.iter_mut().skip(8) {
            *b = small_rng.gen();
        }
        let pong: TestStructVec = LittleEndian(&bytes[..]).decode().unwrap();

        let mut encoder = LittleEndianMut(&mut bytes[..]);
        encoder.encode_into(&pong).unwrap();

        let pong2: TestStructVec = LittleEndian(&bytes[..]).decode().unwrap();

        assert_eq!(pong, pong2);
    }
}
