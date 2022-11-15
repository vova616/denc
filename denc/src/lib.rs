#![feature(associated_type_defaults)]
#![feature(test)]
#![feature(min_specialization)]
#![feature(const_trait_impl)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_try)]
#![feature(const_mut_refs)]
#![feature(const_slice_index)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_replace)]
#![feature(const_convert)]
#![feature(const_maybe_uninit_write)]
#![feature(const_precise_live_drops)]
#![feature(const_maybe_uninit_array_assume_init)]

use std::convert::{TryFrom, TryInto};
use std::io::prelude::{Read, Write};

#[cfg(feature = "derive")]
pub use denc_derive::*;

mod le_decoder;
mod le_encoder;
mod named;

pub use le_decoder::*;
pub use le_encoder::*;
pub use named::*;

const EOF: &'static str = "EOF";

pub trait Decode<T: Decoder>: Sized {
    const SIZE: usize;
    const STATIC: bool = false;

    fn decode(decoder: &mut T) -> Result<Self, T::Error>;

    #[inline]
    fn decode_into(decoder: &mut T, value: &mut Self) -> Result<(), T::Error> {
        *value = Self::decode(decoder)?;
        Ok(())
    }

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

pub trait Encoder: Sized {
    type Error;
    const EOF: Self::Error;

    fn encode<T: Encode<Self>>(&mut self, value: &T) -> Result<usize, Self::Error>;
}
pub trait Decoder: Sized {
    type Error;
    const EOF: Self::Error;

    #[inline]
    fn decode_into<T: Decode<Self>>(&mut self, value: &mut T) -> Result<(), Self::Error> {
        *value = self.decode()?;
        return Ok(());
    }

    #[inline]
    fn decode<T: Decode<Self>>(&mut self) -> Result<T, Self::Error>;
}

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

pub struct BufferedIO<T: Read, V: Sized, const N: usize> {
    reader: T,
    buffer: [V; N],
    cursor: Range<usize>,
    len: usize,
    eof: bool,
}

use core::ops::Range;
use std::mem::{self, MaybeUninit};

impl<T: Read, V: Sized + Default + Copy, const N: usize> BufferedIO<T, V, N> {
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            buffer: [V::default(); N],
            cursor: 0..0,
            len: 0,
            eof: false,
        }
    }
}

impl<T: Read, const N: usize> Read for BufferedIO<T, u8, N> {
    #[inline]
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let buff_len = buf.len();
        if buff_len > self.len {
            let mut len = self.len;
            if len > 0 {
                buf[..len].copy_from_slice(&self.buffer[self.cursor.clone()]);
                self.len = 0;
                self.cursor = 0..0;

                buf = &mut buf[len..];
            }
            if !self.eof {
                let read_len = self.reader.read(&mut self.buffer[..])?;
                if read_len > 0 {
                    let min = buf.len().min(read_len);
                    buf[..min].copy_from_slice(&self.buffer[0..min]);
                    len += min;

                    self.len = read_len - min;
                    self.cursor = min..read_len;
                } else {
                    self.eof = true;
                }
            }
            Ok(len)
        } else {
            buf.copy_from_slice(&self.buffer[self.cursor.clone()][..buff_len]);
            self.len -= buff_len;
            self.cursor.start += buff_len;

            Ok(buff_len)
        }
    }
}

trait InitWith<T, const N: usize> {
    fn init_with<F>(func: F) -> [T; N]
    where
        F: FnMut(usize) -> T;

    fn init_with_result<F, E>(func: F) -> Result<[T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>;
}

impl<T, const N: usize> InitWith<T, N> for [T; N] {
    #[inline(always)]
    fn init_with<F>(mut func: F) -> [T; N]
    where
        F: FnMut(usize) -> T,
    {
        let mut arr: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
        for (i, a) in arr.iter_mut().enumerate() {
            a.write(func(i));
        }
        unsafe { MaybeUninit::array_assume_init(arr) }
    }

    #[inline(always)]
    fn init_with_result<F, E>(mut func: F) -> Result<[T; N], E>
    where
        F: FnMut(usize) -> Result<T, E>,
    {
        let mut arr: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();
        for (i, a) in arr.iter_mut().enumerate() {
            a.write(func(i)?);
        }
        unsafe { Ok(MaybeUninit::array_assume_init(arr)) }
    }
}
