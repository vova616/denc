
#![feature(prelude_import)]
#![feature(test)]
#![feature(specialization)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use denc::*;
pub struct TestStruct {
    pub a: u16,
    pub b: u32,
}
impl<Dec: Decoder> Decode<Dec> for TestStruct {
    const SIZE: usize = <u16 as Decode<Dec>>::SIZE + <u32 as Decode<Dec>>::SIZE;
    #[inline(always)]
    fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error> {
        if decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let b = <u32 as Decode<Dec>>::decode(decoder)?;
        Ok(TestStruct { a, b })
    }
}
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
impl<Dec: Decoder> Decode<Dec> for TestStructSmall {
    const SIZE: usize = <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE;
    #[inline(always)]
    fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error> {
        if decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a87 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a32 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a35 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a23 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a42 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a41 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a7 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a47 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a53 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a25 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a94 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a37 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a11 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a02 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a52 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a43 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a57 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a82 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a01 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a91 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a62 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a26 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a06 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a24 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a71 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a93 = <u32 as Decode<Dec>>::decode(decoder)?;
        Ok(TestStructSmall {
            a87,
            a32,
            a35,
            a23,
            a42,
            a41,
            a7,
            a47,
            a53,
            a25,
            a94,
            a37,
            a11,
            a02,
            a52,
            a43,
            a57,
            a82,
            a01,
            a91,
            a62,
            a26,
            a06,
            a24,
            a71,
            a93,
        })
    }
}
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
impl<Dec: Decoder> Decode<Dec> for TestStructArray {
    const SIZE: usize = <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <[u32; 10] as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE;
    #[inline(always)]
    fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error> {
        if decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a87 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a32 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a35 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a23 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a42 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a41 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <[u32; 10] as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a53 = <[u32; 10] as Decode<Dec>>::decode(decoder)?;
        if !<[u32; 10] as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a7 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a47 = <u8 as Decode<Dec>>::decode(decoder)?;
        Ok(TestStructArray {
            a87,
            a32,
            a35,
            a23,
            a42,
            a41,
            a53,
            a7,
            a47,
        })
    }
}
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
impl<Dec: Decoder> Decode<Dec> for TestStructVec {
    const SIZE: usize = <Vec<u32> as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE;
    #[inline(always)]
    fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error> {
        if decoder.len() < <Vec<u32> as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a53 = <Vec<u32> as Decode<Dec>>::decode(decoder)?;
        if !<Vec<u32> as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a87 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a32 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a35 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a23 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a42 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a41 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a7 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a47 = <u8 as Decode<Dec>>::decode(decoder)?;
        Ok(TestStructVec {
            a53,
            a87,
            a32,
            a35,
            a23,
            a42,
            a41,
            a7,
            a47,
        })
    }
}
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
impl<Dec: Decoder> Decode<Dec> for TestStructLarge {
    const SIZE: usize = <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u8 as Decode<Dec>>::SIZE
        + <u32 as Decode<Dec>>::SIZE
        + <u16 as Decode<Dec>>::SIZE;
    #[inline(always)]
    fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error> {
        if decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a87 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a32 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a35 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a23 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a42 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a41 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a7 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a47 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a53 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a25 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a94 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a37 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a11 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a02 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a52 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a43 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a57 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a82 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a01 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a91 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a62 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a26 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a06 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a24 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a71 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a93 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a34 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a3 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a04 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a65 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a75 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a66 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a17 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a61 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a27 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a13 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a51 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a63 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a92 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a05 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a9 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a14 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a31 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a73 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a6 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a76 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a64 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a97 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a03 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a74 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a46 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a16 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a22 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a21 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a84 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a86 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a95 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a81 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a2 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a44 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a1 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a83 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a96 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a12 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a5 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a55 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a8 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a33 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a15 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a45 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a54 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a77 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a36 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a0 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a07 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a85 = <u16 as Decode<Dec>>::decode(decoder)?;
        if !<u16 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a72 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u8 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a4 = <u8 as Decode<Dec>>::decode(decoder)?;
        if !<u8 as Decode<Dec>>::STATIC && decoder.len() < <u32 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a67 = <u32 as Decode<Dec>>::decode(decoder)?;
        if !<u32 as Decode<Dec>>::STATIC && decoder.len() < <u16 as Decode<Dec>>::SIZE {
            return Err(Dec::EOF);
        }
        let a56 = <u16 as Decode<Dec>>::decode(decoder)?;
        Ok(TestStructLarge {
            a87,
            a32,
            a35,
            a23,
            a42,
            a41,
            a7,
            a47,
            a53,
            a25,
            a94,
            a37,
            a11,
            a02,
            a52,
            a43,
            a57,
            a82,
            a01,
            a91,
            a62,
            a26,
            a06,
            a24,
            a71,
            a93,
            a34,
            a3,
            a04,
            a65,
            a75,
            a66,
            a17,
            a61,
            a27,
            a13,
            a51,
            a63,
            a92,
            a05,
            a9,
            a14,
            a31,
            a73,
            a6,
            a76,
            a64,
            a97,
            a03,
            a74,
            a46,
            a16,
            a22,
            a21,
            a84,
            a86,
            a95,
            a81,
            a2,
            a44,
            a1,
            a83,
            a96,
            a12,
            a5,
            a55,
            a8,
            a33,
            a15,
            a45,
            a54,
            a77,
            a36,
            a0,
            a07,
            a85,
            a72,
            a4,
            a67,
            a56,
        })
    }
}
