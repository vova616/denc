use packed_simd::{u8x16, u8x32, f32x8, u8x64, u8x8};
use std::marker::PhantomData;

pub trait SimdChucks<T> where Self: Sized {

    #[inline(always)]
    fn simd_chunks<F>(slice: &mut [T], mut func: F) where F: FnMut(Self) -> Self;
}


impl SimdChucks<u8> for u8x16 {
    #[inline(always)]
    fn simd_chunks<F>(mut slice: &mut [u8], mut func: F) where F: FnMut(Self) -> Self {
        while slice.len() >= Self::lanes() {
            func(Self::from_slice_unaligned(slice)).write_to_slice_unaligned(&mut slice);
            slice = &mut slice[Self::lanes()..];
        }
        let mut temp = [0u8; Self::lanes()];
        temp[..slice.len()].copy_from_slice(slice);
        func(Self::from_slice_unaligned(&temp)).write_to_slice_unaligned(&mut temp);
        slice.copy_from_slice(&temp[..slice.len()]);
    }
}


impl SimdChucks<u8> for u8x32 {
    #[inline(always)]
    fn simd_chunks<F>(mut slice: &mut [u8], mut func: F) where F: FnMut(Self) -> Self {
        while slice.len() >= Self::lanes() {
            func(Self::from_slice_unaligned(slice)).write_to_slice_unaligned(&mut slice);
            slice = &mut slice[Self::lanes()..];
        }
        let mut temp = [0u8; Self::lanes()];
        temp[..slice.len()].copy_from_slice(slice);
        func(Self::from_slice_unaligned(&temp)).write_to_slice_unaligned(&mut temp);
        slice.copy_from_slice(&temp[..slice.len()]);
    }
}

impl SimdChucks<u8> for u8x64 {
    #[inline(always)]
    fn simd_chunks<F>(mut slice: &mut [u8], mut func: F) where F: FnMut(Self) -> Self {
        while slice.len() >= Self::lanes() {
            func(Self::from_slice_unaligned(slice)).write_to_slice_unaligned(&mut slice);
            slice = &mut slice[Self::lanes()..];
        }
        let mut temp = [0u8; Self::lanes()];
        temp[..slice.len()].copy_from_slice(slice);
        func(Self::from_slice_unaligned(&temp)).write_to_slice_unaligned(&mut temp);
        slice.copy_from_slice(&temp[..slice.len()]);
    }
}


pub struct SimdIter<'a, T,D> {
    slice: &'a [D],
    phantom: std::marker::PhantomData<T>
}

pub trait SimdIterator<'a,T,D>  {

    #[inline(always)]
    fn simd_iter(&'a self) -> SimdIter<'a,T,D>;
}

impl<'a> Iterator for SimdIter<'a,f32x8,f32> {
    type Item = f32x8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.len() >= Self::Item::lanes() {
            let data = Self::Item::from_slice_unaligned(self.slice);
            self.slice = &self.slice[Self::Item::lanes()..];
            Some(data)
        } else if self.slice.len() > 0 {
            let mut temp = [0f32; Self::Item::lanes()];
            temp[..self.slice.len()].copy_from_slice(self.slice);
            self.slice = &self.slice[0..0];
            Some(Self::Item::from_slice_unaligned(&temp))
        } else {
            None
        }
    }
}



impl<'a,T,D> SimdIterator<'a,T,D> for &[D] {
    fn simd_iter(&'a self) -> SimdIter<'a,T,D> {
        SimdIter{ slice: self, phantom: PhantomData }
    }
}

