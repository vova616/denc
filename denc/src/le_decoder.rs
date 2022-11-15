use crate::InitWith;
use crate::{split_at_const, Decode, Decoder, EOF};
use std::convert::{TryFrom, TryInto};
use std::io::prelude::Read;
pub struct LittleEndian<'a>(pub &'a [u8]);

impl<'a> LittleEndian<'a> {
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
    const fn fill_buffer(&mut self, len: usize) -> Result<(), &'static str> {
        if self.0.len() < len {
            Err(EOF)
        } else {
            Ok(())
        }
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
        match decoder.0 {
            &[x, y, ref inner @ ..] => {
                decoder.0 = inner;
                Ok(u16::from_le_bytes([x, y]))
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndian<'a>) -> Result<Self, &'static str> {
        match decoder.0 {
            &[x, y, z, w, ref inner @ ..] => {
                decoder.0 = inner;
                Ok(u32::from_le_bytes([x, y, z, w]))
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 0;

    #[inline(always)]
    fn decode<'b>(decoder: &'b mut LittleEndian<'a>) -> Result<Self, &'static str> {
        Ok(std::mem::replace(&mut decoder.0, &[]))
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
        let size = u32::decode(decoder)? as usize;
        if decoder.0.len() < V::SIZE * size {
            return Err(EOF);
        }
        let mut vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(V::decode(decoder)?);
        }
        Ok(vec)
    }

    #[inline(always)]
    fn decode_into(decoder: &mut LittleEndian<'a>, data: &mut Self) -> Result<(), &'static str> {
        let size = u32::decode(decoder)? as usize;
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
        let size = u32::decode(decoder)? as usize;
        let mut vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(V::decode(decoder)?);
        }
        Ok(vec)
    }

    #[inline(always)]
    fn decode_into(
        decoder: &mut LittleEndianReader<R>,
        data: &mut Self,
    ) -> Result<(), &'static str> {
        let size = u32::decode(decoder)? as usize;
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
