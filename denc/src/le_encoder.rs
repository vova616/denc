use crate::{slice_as_const, split_at_const, split_at_mut_const, Encode, Encoder, EOF};
use core::mem;
use std::convert::{TryFrom, TryInto};
use std::io::prelude::Write;

pub struct LittleEndianMut<'a>(pub &'a mut [u8]);

impl<'a> LittleEndianMut<'a> {
    /*
    #[inline(always)]
    fn advance_exact_const<'b, const N: usize>(
        &'b mut self,
    ) -> Result<&'b mut [u8; N], &'static str> {
        if self.0.len() < N {
            return Err(EOF);
        }
        let (a, b): (&mut [u8; N], &mut [u8]) = split_at_mut_const(self.0).ok_or(EOF)?;
        //self.0 = b;
        //return Ok(a);
        unimplemented("")
    }
    */

    #[inline(always)]
    fn write_const<'b, const N: usize>(&'b mut self, value: &[u8; N]) -> Result<(), &'static str> {
        let slice = std::mem::take(&mut self.0);
        let (a, b) = split_at_mut_const(&mut slice[..]).ok_or(EOF)?;
        self.0 = b;
        *a = *value;
        return Ok(());
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

impl<'a> Encoder for LittleEndianMut<'a> {
    type Error = &'static str;
    const EOF: Self::Error = EOF;

    #[inline]
    fn encode<T: Encode<Self>>(&mut self, value: &T) -> Result<usize, &'static str> {
        if self.0.len() < T::SIZE {
            return Err(EOF);
        }
        let size = self.0.len();
        T::encode(value, self)?;
        let left = self.0.len();
        Ok(size - left)
    }
}

impl<'a> Encode<LittleEndianMut<'a>> for u8 {
    const SIZE: usize = 1;

    #[inline(always)]
    fn encode<'b>(&self, data: &'b mut LittleEndianMut<'a>) -> Result<(), &'static str> {
        data.write_const(&[*self])
    }
}

impl<'a> Encode<LittleEndianMut<'a>> for u16 {
    const SIZE: usize = 2;

    #[inline(always)]
    fn encode<'b>(&self, data: &mut LittleEndianMut<'b>) -> Result<(), &'static str> {
        data.write_const(&self.to_le_bytes())
    }
}

impl<'a> Encode<LittleEndianMut<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn encode<'b>(&self, data: &mut LittleEndianMut<'b>) -> Result<(), &'static str> {
        data.write_const(&self.to_le_bytes())
    }
}

impl<'a> Encode<LittleEndianMut<'a>> for u64 {
    const SIZE: usize = 8;

    #[inline(always)]
    fn encode<'b>(&self, data: &mut LittleEndianMut<'b>) -> Result<(), &'static str> {
        data.write_const(&self.to_le_bytes())
    }
}

impl<'a, V: Encode<LittleEndianMut<'a>>, const N: usize> Encode<LittleEndianMut<'a>> for [V; N] {
    const SIZE: usize = V::SIZE * N;

    #[inline(always)]
    fn encode<'b>(&self, data: &'b mut LittleEndianMut<'a>) -> Result<(), &'static str> {
        data.write_const(&(<[V; N] as Encode<LittleEndianMut<'a>>>::SIZE as u32).to_le_bytes())?;
        for elem in self.iter() {
            if data.0.len() < V::SIZE {
                return Err(EOF);
            }
            V::encode(elem, data)?;
        }
        Ok(())
    }
}

impl<'a, V: Encode<LittleEndianMut<'a>>> Encode<LittleEndianMut<'a>> for Vec<V> {
    const SIZE: usize = V::SIZE;
    const STATIC: bool = false;

    #[inline(always)]
    fn encode<'b>(&self, data: &'b mut LittleEndianMut<'a>) -> Result<(), &'static str> {
        data.write_const(&(self.len() as u32).to_le_bytes())?;
        for elem in self.iter() {
            V::encode(elem, data)?;
        }
        Ok(())
    }
}
