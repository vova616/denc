use packed_simd::{u8x16, u8x32, u8x64};

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



