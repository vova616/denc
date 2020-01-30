#![feature(associated_type_defaults)]
#![feature(generators, generator_trait)]
#![feature(const_generics)]
#![feature(test)]
#![feature(specialization)]
#![feature(const_if_match)]

use smallvec::{smallvec, SmallVec};

use std::convert::{TryFrom, TryInto};
use std::io::prelude::{Read, Write};

#[cfg(feature = "derive")]
pub use denc_derive::*;

mod le_decoder;
mod le_encoder;

pub use le_decoder::*;
pub use le_encoder::*;

/**
One Way:
    size is a size hint
    is should not be too big or the reading wont strat
    before each decoding the size hint ensures that there are enough bytes to
    read the stream

    to decode can read as many bytes he wants

    Why do we need advance:
        I'm not sure.
        maybe for visitors but visitors can implement their own

        Cons: bad impl of advance is bad for visitors.

    Problem with size hint:
        we cannot get final size.
        but we dont need final size when decoding so its fine


*/

const EOF: &'static str = "EOF";

pub trait Decode<T: Decoder>: Sized {
    const SIZE: usize;
    const STATIC: bool = false;

    fn decode<'a>(data: &'a mut T) -> Result<Self, T::Error>;
    fn decode_len(&self) -> usize {
        return Self::SIZE;
    }
}

pub trait Encode<T: Encoder>: Sized {
    const SIZE: usize;
    const STATIC: bool = false;

    fn encode<'a>(&self, data: &'a mut T) -> Result<(), T::Error>;
    fn encode_len(&self) -> usize {
        return Self::SIZE;
    }
}

pub trait Encoder {
    type Error;
    const EOF: Self::Error;

    fn fill_buffer(&mut self, len: usize) -> Result<(), Self::Error>;
    fn len(&self) -> usize;
}

pub trait Decoder {
    type Error;
    const EOF: Self::Error;

    fn fill_buffer(&mut self, len: usize) -> Result<(), Self::Error>;
    fn len(&self) -> usize;
}

/*
impl<V, Dec: Decoder> Decode<Dec> for V {
    default const SIZE: usize = 0;
    default const STATIC: bool = false;

    default fn decode<'a>(data: &'a mut Dec) -> Result<V, Dec::Error> {
        unimplemented!()
    }
}
*/
/*
impl<V, Enc: Encoder> Encode<Enc> for V {
    default const SIZE: usize = 0;

    default fn encode(&self, data: &mut Enc) -> Option<Enc::Error> {
        unimplemented!()
    }

    default fn len(&self) -> usize {
        unimplemented!()
    }
}
*/

/*
impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> &'a [u8] {
        let (x, y) = data.0.split_at(1);
        data.0 = y;
        x
    }
}*/

#[inline(always)]
fn split_at_const<const N: usize>(slice: &[u8]) -> Option<(&[u8; N], &[u8])> {
    // if slice.len() < N {
    //     return None;
    // }
    let r = slice.get(0..N)?;
    let r2 = slice.get(N..)?;
    let ptr = r.as_ptr();
    //cast *u8 to *[u8; N] this should be fine I think?
    let ptr: *const [u8; N] = ptr.cast();
    //dereference ptr
    let ptr = unsafe { &*ptr };
    Some((&ptr, r2))
}

#[inline(always)]
fn split_at_mut_const<const N: usize>(slice: &mut [u8]) -> Option<(&mut [u8; N], &mut [u8])> {
    if slice.len() < N {
        return None;
    }
    let (r, r2) = slice.split_at_mut(N);
    let ptr = r.as_mut_ptr();
    //cast *u8 to *[u8; N] this should be fine I think?
    let ptr: *mut [u8; N] = ptr.cast();
    //dereference ptr
    let ptr = unsafe { &mut *ptr };
    Some((ptr, r2))
}

#[inline(always)]
fn split_at(slice: &[u8], at: usize) -> Option<(&[u8], &[u8])> {
    // if slice.len() < at {
    //     return None;
    // }
    let r = slice.get(0..at)?;
    let r2 = slice.get(at..)?;
    Some((r, r2))
}

#[inline(always)]
fn slice_as_const<const N: usize>(slice: &mut [u8]) -> Option<&mut [u8; N]> {
    if slice.len() < N {
        return None;
    }
    let ptr = slice.as_mut_ptr();
    //cast *u8 to *[u8; N] this should be fine I think?
    let ptr: *mut [u8; N] = ptr.cast();
    //dereference ptr
    let ptr = unsafe { &mut *ptr };
    Some(ptr)
}
