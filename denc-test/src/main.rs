#![feature(test)]
#![feature(min_specialization)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

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

#[derive(Default, Enc)]
pub struct TestStruct2 {
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
    let mut small_rng = SmallRng::from_seed([15u8; 16]);
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

    pub struct TestStruct {
        pub a: u16,
        pub b: u32,
    }

    impl Dic for TestStruct {
        const FIELDS: &'static [&'static str] = &["a", "b"];
    }

    #[test]
    fn test() {
        dbg!(TestStruct::from_str(&"a"));
        dbg!(TestStruct::from_str(&"b"));
    }
    

    macro_rules! test_struct {
        ($func:ident, $typ:ident, $size:expr) => {
            #[test]
            fn $func() {
                let mut small_rng = SmallRng::from_seed([15u8; 16]);
                let mut bytes = vec![0u8; $size];
                bytes[0] = 10;
                for b in bytes.iter_mut().skip(8) {
                    *b = small_rng.gen();
                }
                let mut decoder = LittleEndian(&bytes[..]);
                let pong: $typ = decoder.decode().unwrap();

                let mut output = vec![0u8; $size];
                let mut encoder = LittleEndianMut(&mut output[..]);
                let size = encoder.encode(&pong).unwrap();

                assert_eq!(&output[..size], &bytes[..size])
            }
        };
    }

    test_struct!(test1_small, TestStructSmall, 1024);
    test_struct!(test1_large, TestStructLarge, 1024);
    test_struct!(test1_array, TestStructArray, 1024);
    test_struct!(test1_vec, TestStructVec, 1024);
}

#[cfg(test)]
mod benches {
    use super::*;
    use std::io::BufReader;

    macro_rules! bench_decode {
        ($func:ident, $typ:ident, $size:expr) => {
            #[bench]
            fn $func(b: &mut Bencher) {
                let mut small_rng = SmallRng::from_seed([15u8; 16]);
                let mut bytes = vec![0u8; $size];
                bytes[0] = 10;
                for b in bytes.iter_mut().skip(8) {
                    *b = small_rng.gen();
                }
                b.iter(|| {
                    test::black_box(&bytes);
                    let mut bytes = LittleEndian(&bytes[..]);
                    let pong: $typ = bytes.decode().unwrap();
                    test::black_box(pong);
                });
            }
        };
    }

    macro_rules! bench_decode_into {
        ($func:ident, $typ:ident, $size:expr) => {
            #[bench]
            fn $func(b: &mut Bencher) {
                let mut small_rng = SmallRng::from_seed([15u8; 16]);
                let mut bytes = vec![0u8; $size];
                bytes[0] = 10;
                for b in bytes.iter_mut().skip(8) {
                    *b = small_rng.gen();
                }
                b.iter(|| {
                    test::black_box(&bytes);
                    let mut bytes = LittleEndian(&bytes[..]);
                    let mut pong: $typ = Default::default();
                    bytes.decode_into(&mut pong).unwrap();
                    test::black_box(&pong);
                });
            }
        };
    }

    macro_rules! bench_decode_reader {
        ($func:ident, $typ:ident, $size:expr) => {
            #[bench]
            fn $func(b: &mut Bencher) {
                let mut small_rng = SmallRng::from_seed([15u8; 16]);
                let mut bytes = vec![0u8; $size];
                bytes[0] = 10;
                for b in bytes.iter_mut().skip(8) {
                    *b = small_rng.gen();
                }
                b.iter(|| {
                    test::black_box(&bytes);
                    let mut bytes = LittleEndianReader::new(&bytes[..] as &[u8]);
                    let pong: $typ = bytes.decode().unwrap();
                    test::black_box(pong);
                });
            }
        };
    }

    

    macro_rules! bench_encode {
        ($func:ident, $typ:ident, $size:expr) => {
            #[bench]
            fn $func(b: &mut Bencher) {
                let mut small_rng = SmallRng::from_seed([15u8; 16]);
                let mut bytes = vec![0u8; $size];
                bytes[0] = 10;
                for b in bytes.iter_mut().skip(8) {
                    *b = small_rng.gen();
                }
                let pong: $typ = LittleEndian(&bytes[..]).decode().unwrap();
                b.iter(|| {
                    test::black_box(&pong);
                    let mut encoder = LittleEndianMut(&mut bytes[..]);
                    encoder.encode(&pong).unwrap();
                    test::black_box(&bytes[..]);
                });
            }
        };
    }

    macro_rules! bench_struct {
        ($name:ident, $typ:ident, $size:expr) => {
            mod $name {
                use super::*;
                mod decode_reader {
                    use super::*;
                    bench_decode_reader!($name, $typ, $size);
                }
                mod decode_into {
                    use super::*;
                    bench_decode_into!($name, $typ, $size);
                }
                mod decode {
                    use super::*;
                    bench_decode!($name, $typ, $size);
                }
                mod encode {
                    use super::*;
                    bench_encode!($name, $typ, $size);
                }
            }
        };
    }
    
    bench_struct!(small, TestStructSmall, 1024);
    bench_struct!(large, TestStructLarge, 1024);
    bench_struct!(array, TestStructArray, 1024);
    bench_struct!(vec, TestStructVec, 1024);



    use std::ops::Range;


    const fn test() {
        let arr = [1,2,3,4];
        let le = &mut LittleEndian(&arr);
        let le1: u8 = match le.decode() {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };
        let le2: u16 = match le.decode() {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };
        let r: () = if le1 != 1 { panic!("u8") };
        let r2: () = if le2 != 0x302 { panic!("u16") };
    }

    const A: () = test();
}
