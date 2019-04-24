#![feature(associated_type_defaults)]
#![feature(generators, generator_trait)]
#![feature(const_generics)]
#![feature(test)]

pub mod list;
pub mod little_endian;

use smallvec::{SmallVec, smallvec};

use std::convert::{TryInto,TryFrom};
pub use list::{RefList, List};
use std::io::prelude::{Write, Read};

#[cfg(feature = "derive")]
pub use mapper_derive::*;

pub trait Encoder {
    #[inline(always)]
    fn encode(&self) -> SmallVec<[u8; 1024]> {
        let mut buffer = smallvec![0u8; self.size_enc()];
        self.encode_into(&mut &mut buffer[..]);
        buffer
    }

    #[inline(always)]
    fn encode_into<T : Write>(&self, buff: &mut T) ;

    #[inline(always)]
    fn size_enc(&self) -> usize;
}

pub trait Decoder<'b> {
    type Output;

    #[inline(always)]
    fn decode(reader: &'b [u8]) -> Self::Output;

    #[inline(always)]
    fn size(buffer: &'b [u8]) -> usize;
}


#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;
    use rand::SeedableRng;
    use rand::FromEntropy;
    use rand::rngs::SmallRng;
    use rand::Rng;

    #[derive(MapperDec, MapperEnc)]
    pub struct TestStructTiny {
        pub a87: u16,
        pub a32: u32,
        pub a35: u32,
        pub a23: u16,
        pub a42: u8,
    }

    #[derive(MapperDec, MapperEnc)]
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

    #[derive(MapperDec, MapperEnc)]
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
        pub a56: u16
    }

    #[derive(MapperDec, MapperEnc)]
    pub struct TestStructArrays {
        pub a87: [u16; 3],
        pub a42: [u8; 7],
        pub a35: [u32; 3],
        pub a23: [u16; 3],
    }

    #[derive(MapperDec, MapperEnc)]
    pub struct TestStructLists {
        pub a87: List<u16, u8>,
        pub a42: List<u32, u8>,
        pub a35: List<u8, u8>,
        pub a23: List<u16, u8>,
    }

    #[derive(MapperDec, MapperEnc)]
    pub struct TestStructRefLists<'a> {
        pub a87: RefList<'a, u16, u8>,
        pub a42: RefList<'a, u32, u8>,
        pub a35: RefList<'a, u8, u8>,
        pub a23: RefList<'a, u16, u8>,
    }

    #[bench]
    fn bench_decode_tiny(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                let mut pong = TestStructTiny::decode(test::black_box(&bytes));
                test::black_box(pong);
            });
        });
    }


    #[bench]
    fn bench_decode_small(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                let mut pong = TestStructSmall::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_long(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 200];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                let mut pong = TestStructLarge::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_arrays(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                let mut pong = TestStructArrays::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_lists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 5;
            }
            b.iter(||  {
                let mut pong = TestStructLists::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_reflists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 5;
            }
            b.iter(||  {
                let mut pong = TestStructRefLists::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_big_lists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1000];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 20;
            }
            b.iter(||  {
                let mut pong = TestStructLists::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_decode_big_reflists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1000];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 20;
            }
            b.iter(||  {
                let mut pong = TestStructRefLists::decode(&bytes);
                test::black_box(pong);
            });
        });
    }

    #[bench]
    fn bench_encode_small(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let mut pong = TestStructSmall::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }

    #[bench]
    fn bench_encode_long(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 200];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let mut pong = TestStructLarge::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }

    #[bench]
    fn bench_encode_arrays(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let mut pong = TestStructArrays::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }

    #[bench]
    fn bench_encode_lists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 5;
            }
            let mut pong = TestStructLists::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }

    #[bench]
    fn bench_encode_reflists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 100];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 5;
            }
            let mut pong = TestStructRefLists::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }

    #[bench]
    fn bench_encode_big_lists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1000];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 20;
            }
            let mut pong = TestStructLists::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }

    #[bench]
    fn bench_encode_big_reflists(b: &mut Bencher) {
        let mut small_rng = SmallRng::seed_from_u64(123456);
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 1000];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
                *b %= 20;
            }
            let mut pong = TestStructRefLists::decode(&bytes);
            b.iter(||  {
                pong.encode();
            });
        });
    }
}


