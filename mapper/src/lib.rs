#![feature(associated_type_defaults)]
#![feature(generators, generator_trait)]
#![feature(const_generics)]


pub mod list;
pub mod little_endian;



use std::convert::{TryInto,TryFrom};
pub use list::{RefList, List};
use std::io::prelude::{Write, Read};

pub trait Encoder {

    #[inline(always)]
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size_enc());
        self.encode_into(&mut buffer);
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




