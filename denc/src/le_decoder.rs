use crate::InitWith;
use crate::{split_at_const, Decode, Decoder, EOF};
use std::convert::{TryFrom, TryInto};
use std::io::prelude::Read;

pub struct LittleEndian<'a>(pub &'a [u8]);

use std::ops::Range;

impl<'a> LittleEndian<'a> {
    #[inline(always)]
    fn advance(&mut self, len: usize) -> Option<()> {
        self.0 = self.0.get(len..)?;
        Some(())
    }

    #[inline]
    pub fn from_slice<T: Decode<Self> + Default>(slice: &'a [u8]) -> Result<T, &'static str> {
        LittleEndian(slice).decode()
    }

    #[inline]
    pub fn from_reader<R: Read, T: Decode<LittleEndianReader<R>> + Default>(
        reader: R,
    ) -> Result<T, &'static str> {
        LittleEndianReader { reader }.decode()
    }

    #[inline]
    pub fn decode_into<T: Decode<Self>>(&mut self, value: &mut T) -> Result<(), &'static str> {
        self.fill_buffer(T::SIZE)?;
        T::decode_into(self, value)?;
        Ok(())
    }

    #[inline(always)]
    fn buff_advance_exact_const<'b, const N: usize>(&'b mut self) -> Option<&'b [u8; N]> {
        let (next, new) = split_at_const::<N>(self.0)?;
        self.0 = new;
        return Some(next);
    }

    #[inline(always)]
    fn get_const<'b, const N: usize>(&'b mut self) -> Option<&'b [u8; N]> {
        let ptr = self.0.get(0..N)?.as_ptr();
        //cast *u8 to *[u8; N] this should be fine I think?
        let ptr: *const [u8; N] = ptr.cast();
        //dereference ptr
        let ptr = unsafe { &*ptr };
        Some(&ptr)
    }

    #[inline(always)]
    fn buff_advance_exact<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        if self.0.len() < len {
            return None;
        }
        let r = self.0.get(0..len)?;
        self.0 = self.0.get(len..)?;
        return Some(r);
    }

    #[inline(always)]
    fn read_const<'b, const N: usize>(&'b mut self) -> Result<&[u8; N], &'static str> {
        let (a, b) = split_at_const(&self.0).ok_or(EOF)?;
        self.0 = b;
        return Ok(a);
    }

    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) -> Result<(), &'static str> {
        if self.0.len() < len {
            Err(EOF)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Decoder for LittleEndian<'a> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

    #[inline]
    fn decode<T: Decode<Self>>(&mut self) -> Result<T, Self::Error> {
        self.fill_buffer(T::SIZE)?;
        T::decode(self)
    }
}

impl<'a> Decode<LittleEndian<'a>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndian<'a>) -> Result<Self, &'static str> {
        match decoder.0 {
            &[x, ref inner @ ..] => {
                decoder.0 = inner;
                Ok(x)
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(decoder: &'b mut LittleEndian<'a>) -> Result<Self, &'static str> {
        let slice = decoder.buff_advance_exact(2).ok_or(EOF)?;
        Ok(u16::from_le_bytes(slice.try_into().ok().ok_or(EOF)?))
    }
}

impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndian<'a>) -> Result<Self, &'static str> {
        let slice = decoder.buff_advance_exact(4).ok_or(EOF)?;
        Ok(u32::from_le_bytes(slice.try_into().ok().ok_or(EOF)?))
    }
}

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 0;

    #[inline(always)]
    fn decode<'b>(decoder: &'b mut LittleEndian<'a>) -> Result<Self, &'static str> {
        Ok(&decoder.0.get(..).ok_or(EOF)?)
    }
}

impl<'a, V: Decode<LittleEndian<'a>>, const N: usize> Decode<LittleEndian<'a>> for [V; N] {
    const SIZE: usize = V::SIZE * N;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndian<'a>) -> Result<Self, &'static str> {
        <Self>::init_with_result(|i| V::decode(decoder))
    }

    #[inline(always)]
    fn decode_into(decoder: &mut LittleEndian<'a>, data: &mut Self) -> Result<(), &'static str> {
        for elem in data.iter_mut() {
            V::decode_into(decoder, elem)?;
        }
        Ok(())
    }
}

impl<'a, V: Decode<LittleEndian<'a>> + Default> Decode<LittleEndian<'a>> for Vec<V> {
    const SIZE: usize = <u32 as Decode<LittleEndian<'a>>>::SIZE;
    const STATIC: bool = false;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndian<'a>) -> Result<Self, &'static str> {
        let mut vec = Vec::new();
        Self::decode_into(decoder, &mut vec)?;
        Ok(vec)
    }

    #[inline(always)]
    fn decode_into(decoder: &mut LittleEndian<'a>, data: &mut Self) -> Result<(), &'static str> {
        let mut size: u32 = u32::decode(decoder)?;
        let size = size as usize;
        if decoder.0.len() < V::SIZE * size {
            return Err(EOF);
        }
        data.clear();
        if data.capacity() < size {
            data.reserve(size - data.capacity());
        }
        for _ in 0..size {
            data.push(V::decode(decoder)?);
        }
        Ok(())
    }
}

pub struct LittleEndianReader<R: Read> {
    pub reader: R,
}

impl<R: Read> LittleEndianReader<R> {
    pub fn new(reader: R) -> Self {
        LittleEndianReader { reader }
    }

    pub fn inner(self) -> R {
        self.reader
    }

    /*
        Next Refactor:
        Remove fill_buffer from each line?
    */
}

impl<R: Read> Decoder for LittleEndianReader<R> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

    #[inline]
    fn decode<T: Decode<Self>>(&mut self) -> Result<T, Self::Error> {
        return T::decode(self);
    }
}

impl<R: Read> Decode<LittleEndianReader<R>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndianReader<R>) -> Result<Self, &'static str> {
        let mut bytes = [0u8; 1];
        decoder.reader.read_exact(&mut bytes[..]).map_err(|e| EOF)?;
        Ok(bytes[0])
    }
}

impl<R: Read> Decode<LittleEndianReader<R>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndianReader<R>) -> Result<Self, &'static str> {
        let mut bytes = [0u8; 2];
        decoder.reader.read_exact(&mut bytes[..]).map_err(|e| EOF)?;
        Ok(u16::from_le_bytes(bytes))
    }
}

impl<R: Read> Decode<LittleEndianReader<R>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndianReader<R>) -> Result<Self, &'static str> {
        let mut bytes = [0u8; 4];
        decoder.reader.read_exact(&mut bytes[..]).map_err(|e| EOF)?;
        Ok(u32::from_le_bytes(bytes))
    }
}

impl<R: Read, V: Decode<LittleEndianReader<R>>, const N: usize> Decode<LittleEndianReader<R>>
    for [V; N]
{
    const SIZE: usize = if V::SIZE * N > 1024 {
        V::SIZE
    } else {
        V::SIZE * N
    };
    const STATIC: bool = V::SIZE * N <= 1024;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndianReader<R>) -> Result<Self, &'static str> {
        <Self>::init_with_result(|i| V::decode(decoder))
    }

    #[inline(always)]
    fn decode_into(
        decoder: &mut LittleEndianReader<R>,
        data: &mut Self,
    ) -> Result<(), &'static str> {
        for elem in data.iter_mut() {
            V::decode_into(decoder, elem)?;
        }
        Ok(())
    }
}

impl<R: Read, V: Decode<LittleEndianReader<R>>> Decode<LittleEndianReader<R>> for Vec<V> {
    const SIZE: usize = <u32 as Decode<LittleEndianReader<R>>>::SIZE;
    const STATIC: bool = false;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndianReader<R>) -> Result<Self, &'static str> {
        let mut vec = Vec::new();
        Self::decode_into(decoder, &mut vec)?;
        Ok(vec)
    }

    #[inline(always)]
    fn decode_into(
        decoder: &mut LittleEndianReader<R>,
        data: &mut Self,
    ) -> Result<(), &'static str> {
        let mut size: u32 = u32::decode(decoder)?;
        let size = size as usize;
        data.clear();
        if data.capacity() < size {
            data.reserve(size - data.capacity());
        }
        for _ in 0..size {
            data.push(V::decode(decoder)?);
        }
        Ok(())
    }
}
