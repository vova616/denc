use crate::{split_at_const, Decode, Decoder, EOF};
use std::convert::{TryFrom, TryInto};
use std::io::prelude::Read;

pub struct LittleEndian<'a>(pub &'a [u8]);

impl<'a> LittleEndian<'a> {
    #[inline(always)]
    fn advance(&mut self, len: usize) -> Option<()> {
        self.0 = self.0.get(len..)?;
        Some(())
    }

    #[inline]
    pub fn decode<T: Decode<Self>>(&mut self) -> Result<T, &'static str> {
        self.fill_buffer(T::SIZE)?;
        T::decode(self)
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
}

impl<'a> Decoder for LittleEndian<'a> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

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

impl<'a> Decode<LittleEndian<'a>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<u8, &'static str> {
        match data.0 {
            &[x, ref inner @ ..] => {
                data.0 = inner;
                Ok(x)
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<u16, &'static str> {
        match data.0 {
            &[x1, x2, ref inner @ ..] => {
                data.0 = inner;
                Ok(u16::from_le_bytes([x1, x2]))
            }
            _ => Err(EOF),
        }
    }
}

impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<u32, &'static str> {
        //this is slower for some reason????
        // match data.0 {
        //     &[x1, x2, x3, x4, ref inner @ ..] => {
        //         data.0 = inner;
        //         Ok(u32::from_le_bytes([x1, x2, x3, x4]))
        //     }
        //     _ => Err(EOF),
        // }
        let slice = data.buff_advance_exact(4).ok_or(EOF)?;
        Ok(u32::from_le_bytes(slice.try_into().ok().ok_or(EOF)?))
    }
}

impl<'a> Decode<LittleEndian<'a>> for &'a [u8] {
    const SIZE: usize = 0;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<&'a [u8], &'static str> {
        Ok(&data.0.get(..).ok_or(EOF)?)
    }
}

use std::mem::{self, MaybeUninit};
impl<'a, V: Decode<LittleEndian<'a>> + Default + Sized + Copy, const N: usize>
    Decode<LittleEndian<'a>> for [V; N]
{
    const SIZE: usize = V::SIZE * N;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<[V; { N }], &'static str> {
        let mut arr: [MaybeUninit<V>; { N }] = unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut arr[..N] {
            if data.len() < V::SIZE {
                return Err(EOF);
            }
            *elem = MaybeUninit::new(V::decode(data)?);
        }
        let arr: *const [V; { N }] = arr.as_ptr().cast();
        Ok(unsafe { *arr })
    }
}

impl<'a, V: Decode<LittleEndian<'a>>> Decode<LittleEndian<'a>> for Vec<V> {
    const SIZE: usize = <u32 as Decode<LittleEndian<'a>>>::SIZE;
    //const STATIC: bool = false;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndian<'a>) -> Result<Vec<V>, &'static str> {
        let size: usize = <u32 as Decode<LittleEndian<'a>>>::decode(data)? as usize;
        let mut arr = Vec::<V>::with_capacity(size as usize);
        if data.len() < V::SIZE * size {
            return Err(EOF);
        }
        for _ in 0..size {
            arr.push(V::decode(data)?);
        }
        //<&[u8;4]>
        Ok(arr)
    }
}

use std::ops::Range;

pub struct LittleEndianReader<'a, R: Read> {
    pub reader: R,
    pub buffer: &'a mut [u8],
    pub cursor: Range<usize>,
}

impl<'a, R: Read> LittleEndianReader<'a, R> {
    pub fn new(reader: R, buffer: &'a mut [u8]) -> Self {
        LittleEndianReader {
            reader,
            buffer,
            cursor: 0..0,
        }
    }

    fn inner(self) -> R {
        self.reader
    }

    fn buff<'b>(&'b self) -> &'b [u8] {
        &self.buffer[self.cursor.clone()]
    }

    /*
        Next Refactor:
        Remove fill_buffer from each line?
    */

    #[inline(always)]
    fn buff_advance<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        self.fill_buffer(len).ok()?;
        let buff = self.buffer.get(self.cursor.clone())?;
        self.cursor.start += len;
        Some(buff)
    }

    #[inline(always)]
    fn buff_advance_exact<'b>(&'b mut self, len: usize) -> Option<&'b [u8]> {
        self.fill_buffer(len).ok()?;
        let buff = self
            .buffer
            .get(self.cursor.start..self.cursor.start + len)?;
        self.cursor.start += len;
        Some(buff)
    }

    #[inline]
    pub fn decode<T: Decode<Self>>(&mut self) -> Result<T, &'static str> {
        self.fill_buffer(T::SIZE)?;
        T::decode(self)
    }

    #[inline]
    pub fn fill_buffer_inner(&mut self, len: usize) -> Result<(), &'static str> {
        if self.buffer.len() < len + self.cursor.start {
            if self.buffer.len() < len {
                return Err("Buffer is too small");
            }
            self.buffer.copy_within(self.cursor.clone(), 0);
            self.cursor = 0..self.cursor.len();
        }
        self.cursor.end += match self.reader.read(&mut self.buffer[self.cursor.end..]) {
            Ok(n) => n,
            Err(e) => return Err("Read err"),
        };
        Ok(())
    }
}

impl<'a, R: Read> Decoder for LittleEndianReader<'a, R> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

    #[inline(always)]
    fn fill_buffer(&mut self, len: usize) -> Result<(), &'static str> {
        while self.cursor.len() < len {
            self.fill_buffer_inner(len)?;
        }
        Ok(())
        //assert!(self.cursor.len() >= len);
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.cursor.len()
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<u8, &'static str> {
        let r = data.buff_advance_exact(1).ok_or(EOF)?.get(0).ok_or(EOF)?;
        Ok(*r)
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<u16, &'static str> {
        let buff = data.buff_advance_exact(2).ok_or(EOF)?;
        let r = u16::from_le_bytes(buff.try_into().ok().ok_or(EOF)?);
        Ok(r)
    }
}

impl<'a, R: Read> Decode<LittleEndianReader<'a, R>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<u32, &'static str> {
        let buff = data.buff_advance_exact(4).ok_or(EOF)?;
        let r = u32::from_le_bytes(buff.try_into().ok().ok_or(EOF)?);
        Ok(r)
    }
}

impl<'a, R: Read, V: Decode<LittleEndianReader<'a, R>> + Copy, const N: usize>
    Decode<LittleEndianReader<'a, R>> for [V; N]
{
    const SIZE: usize = if V::SIZE * N > 1024 {
        V::SIZE
    } else {
        V::SIZE * N
    };
    const STATIC: bool = V::SIZE * N <= 1024;

    #[inline(always)]
    fn decode<'b>(data: &'b mut LittleEndianReader<'a, R>) -> Result<[V; { N }], &'static str> {
        let mut arr: [MaybeUninit<V>; { N }] = unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut arr[..] {
            *elem = MaybeUninit::new(V::decode(data)?);
        }
        let arr: *const [V; { N }] = arr.as_ptr().cast();
        Ok(unsafe { *arr })
    }
}
