#![feature(associated_type_defaults)]
#![feature(generators, generator_trait)]
#![feature(const_generics)]


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




