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

    use mapper::{Decoder,Encoder, List, RefList,  MapperDec, MapperEnc};
    use rand::SeedableRng;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn decrypt_simd() {
        let mut bytes = [0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut bytes_test = bytes.clone();
        cryptor::decrypt_simd16(&mut bytes[..]);
        cryptor::decrypt(&mut bytes_test[..]);
        assert_eq!(&bytes[..], &bytes_test[..]);


        let mut bytes = [0x70, 0xAF, 0x8D, 0x6A, 0x00, 0x93, 0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x3D, 0x1B, 0x18, 0x13, 0x0F, 0x03, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x00, 0x00, ];
        let mut bytes_test = bytes.clone();
        cryptor::decrypt_simd16(&mut bytes[..]);
        cryptor::decrypt(&mut bytes_test[..]);
        assert_eq!(&bytes[..], &bytes_test[..]);

        let mut bytes = [0x70, 0xAF, 0x8D, 0x6A, 0x00, 0x93, 0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x3D, 0x1B, 0x18, 0x13, 0x0F, 0x03, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1, 0x00, 0x00, 0x3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x00, 0x00, ];
        let mut bytes_test = bytes.clone();
        cryptor::decrypt_simd16(&mut bytes[..]);
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
            cryptor::decrypt_simd16(&mut bytes_test[..]);
            cryptor::decrypt_simd32(&mut bytes_test2[..]);
            cryptor::decrypt_hybrid_16(&mut bytes_test3[..]);
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
            let mut bytes_test2 = bytes.clone();
            let mut bytes_test3 = bytes.clone();
            cryptor::decrypt(&mut bytes[..]);
            cryptor::decrypt_hybrid_16(&mut bytes_test[..]);
            cryptor::decrypt_hybrid_32(&mut bytes_test2[..]);
            cryptor::decrypt_hybrid_64(&mut bytes_test3[..]);
            assert_eq!(&bytes[..], &bytes_test[..]);
            assert_eq!(&bytes[..], &bytes_test2[..]);
            assert_eq!(&bytes[..], &bytes_test3[..]);
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
                cryptor::decrypt( test::black_box(&mut bytes[..]));
                test::black_box(&bytes);
            });
        });
    }


    #[bench]
    fn bench_decrypt_simd16(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 32*5];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt_simd16( test::black_box(&mut bytes[..]));
                test::black_box(&bytes);
            });
        });
    }

    #[bench]
    fn bench_decrypt_simd32(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|_| {
            let mut bytes = vec![0u8; 32*5];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt_simd32( test::black_box(&mut bytes[..]));
                test::black_box(&bytes);
            });
        });
    }


    #[bench]
    fn bench_decrypt_hybrid_16(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|i| {
            let mut bytes = vec![0u8; 32*5-1];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt_hybrid_16( test::black_box(&mut bytes[..]));
                test::black_box(&bytes);
            });
        });
    }

    #[bench]
    fn bench_decrypt_hybrid_32(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|i| {
            let mut bytes = vec![0u8; 32*5-1];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt_hybrid_32( test::black_box(&mut bytes[..]));
                test::black_box(&bytes);
            });
        });
    }

    #[bench]
    fn bench_decrypt_hybrid_64(b: &mut Bencher) {
        let mut small_rng = SmallRng::from_entropy();
        (0..5).for_each(|i| {
            let mut bytes = vec![0u8; 32*5-1];
            for b in bytes.iter_mut() {
                *b = small_rng.gen();
            }
            b.iter(||  {
                cryptor::decrypt_hybrid_64( test::black_box(&mut bytes[..]));
                test::black_box(&bytes);
            });
        });
    }



}