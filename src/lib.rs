#![feature(test)]
#![feature(const_fn)]

#[macro_use]
extern crate lazy_static;


extern crate test;
use std::collections::HashSet;


pub fn add_two(a: i32) -> i32 {
    a + 2
}

mod cryptor;

use radix_fmt::radix;
use rand::FromEntropy;
use rand::rngs::SmallRng;
use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn decrypt_simd() {
        let mut bytes = [0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut bytes_test = bytes.clone();
        cryptor::decrypt2(&mut bytes[..]);
        cryptor::decrypt(&mut bytes_test[..]);
        assert_eq!(&bytes[..], &bytes_test[..]);


        let mut bytes = [0x70, 0xAF, 0x8D, 0x6A, 0x00, 0x93, 0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x3D, 0x1B, 0x18, 0x13, 0x0F, 0x03, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x00, 0x00, ];
        let mut bytes_test = bytes.clone();
        cryptor::decrypt2(&mut bytes[..]);
        cryptor::decrypt(&mut bytes_test[..]);
        assert_eq!(&bytes[..], &bytes_test[..]);

        let mut bytes = [0x70, 0xAF, 0x8D, 0x6A, 0x00, 0x93, 0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x3D, 0x1B, 0x18, 0x13, 0x0F, 0x03, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1, 0x00, 0x00, 0x3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x00, 0x00, ];
        let mut bytes_test = bytes.clone();
        cryptor::decrypt2(&mut bytes[..]);
        cryptor::decrypt(&mut bytes_test[..]);
        assert_eq!(&bytes[..], &bytes_test[..]);

        let mut small_rng = SmallRng::from_entropy();
        (0..10).for_each(|_| {
            let mut bytes = vec![0u8; 32*32];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let mut bytes_test = bytes.clone();
            let mut bytes_test2 = bytes.clone();
            let mut bytes_test3 = bytes.clone();
            cryptor::decrypt(&mut bytes[..]);
            cryptor::decrypt2(&mut bytes_test[..]);
            cryptor::decrypt3(&mut bytes_test2[..]);
            cryptor::decrypt_hybrid(&mut bytes_test3[..]);
            assert_eq!(&bytes[..], &bytes_test[..]);
            assert_eq!(&bytes[..], &bytes_test2[..]);
            assert_eq!(&bytes[..], &bytes_test3[..]);
        });

        (0..10).for_each(|_| {
            let mut bytes = vec![0u8; 1011];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            let mut bytes_test = bytes.clone();
            cryptor::decrypt(&mut bytes[..]);
            cryptor::decrypt_hybrid(&mut bytes_test[..]);
            assert_eq!(&bytes[..], &bytes_test[..]);
        });
    }


    #[bench]
    fn bench_decrypt(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 32*5];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt(&mut bytes[..]);
                test::black_box(&bytes);
            });
        });
    }

    #[bench]
    fn bench_decrypt2(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 32*5];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt2(&mut bytes[..]);
                test::black_box(&bytes);
            });
        });
    }

    #[bench]
    fn bench_decrypt3(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 32*5];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt3(&mut bytes[..]);
                test::black_box(&bytes);
            });
        });
    }

    #[bench]
    fn bench_decrypt_hybrid(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|i| {
            let mut bytes = vec![0u8; 32*5 + i * 11];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt_hybrid(&mut bytes[..]);
                test::black_box(&bytes);
            });
        });
    }




    use mapper::{Decoder,Encoder, List, RefList,  MapperDec, MapperEnc};
    use rand::SeedableRng;

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



    #[test]
    fn it_works2() {
        println!("{}", radix(gen(100), 36));
    }


    fn bench_gen(b: &mut Bencher) {
        b.iter(||  {
            gen(1000000);
        });
    }

    fn gen(target: u64) -> u64 {
        let mut small_rng = SmallRng::from_entropy();
        let mut set = HashSet::with_capacity(target as usize);
        let size = 10;

        let mut target = target+1;
        let mut num  = 0u64;
        let low = 0;
        let max = 36u64.pow(size);
        let mut duplicates = 0;
        while target > 0 {
            num = small_rng.gen_range(0u64, max);
            target -= set.insert(num) as u64;
        }
        num
    }

}