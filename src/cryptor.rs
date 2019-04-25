use packed_simd::{u8x16, u8x32, u8x64,  shuffle};
use packed_simd::FromCast;
use packed_simd::Cast;
use packed_simd::{FromBits, IntoBits};

use simd_chunks::SimdChucks;

const KEY: &[u8; 11] = b"qmfaktnpgjs";

#[inline(always)]
pub fn decrypt(buf: &mut [u8]) {
    buf.iter_mut().zip(KEY.iter().cycle()).for_each(|(data, &k)| {
        if !(*data == 0 || *data == k) {
            *data ^= k;
        }
    });
}

#[inline(always)]
pub fn decrypt_simd16(buff: &mut [u8]) {
    assert_eq!(buff.len() % 16, 0);
    let mut key =  u8x16::new(0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,0x71,0x6d,0x66,0x61,0x6b);
    let zero = u8x16::splat(0);
    for mut slice in buff.chunks_exact_mut(u8x16::lanes()) {
        let x = u8x16::from_slice_unaligned(slice);
        let y = key.eq(x) | zero.eq(x);
        let b = y.select(x, x ^ key);
        b.write_to_slice_unaligned(&mut slice);
        key = shuffle!(key, [5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9])
    }
}

#[inline(always)]
pub fn decrypt_simd32(buff: &mut [u8]) {
    assert_eq!(buff.len() % 32, 0);
    let mut key =  u8x32::new(0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a);
    let zero = u8x32::splat(0);
    for mut slice in buff.chunks_exact_mut(u8x32::lanes()) {
        let data = u8x32::from_slice_unaligned(slice);
        let y = key.eq(data) | zero.eq(data);
        let data = y.select(data, data ^ key);
        data.write_to_slice_unaligned(&mut slice);
        key = shuffle!(key, [10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8])
    }
}

#[inline(always)]
pub fn decrypt_hybrid_16(buff: &mut [u8]) {
    let mut key =  u8x16::new(0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,0x71,0x6d,0x66,0x61,0x6b);
    u8x16::simd_chunks(buff, |data| {
        let y = key.eq(data) | u8x16::splat(0).eq(data);
        let data = y.select(data, data ^ key);
        key = shuffle!(key,  [5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9]);
        data
    });
}

#[inline(always)]
pub fn decrypt_hybrid_32(buff: &mut [u8]) {
    let mut key =  u8x32::new(0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a);
    u8x32::simd_chunks(buff, |data| {
        let y = key.eq(data) | u8x32::splat(0).eq(data);
        let data = y.select(data, data ^ key);
        key = shuffle!(key,  [10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8]);
        data
    });
}

#[inline(always)]
pub fn decrypt_hybrid_64(buff: &mut [u8]) {
    let mut key =  u8x64::new(0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,
                              0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67);
    u8x64::simd_chunks(buff, |data| {
        let y = key.eq(data) | u8x64::splat(0).eq(data);
        let data = y.select(data, data ^ key);
        key = shuffle!(key,  [9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6]);
        data
    });
}



#[inline(always)]
pub fn encrypt(buf: &mut [u8]) {
    buf.iter_mut().zip(KEY.iter().cycle()).for_each(|(data, &k)| {
        if !(*data == 0 || *data == k) {
            *data ^= k;
        }
    });
}

pub fn xor(buff: &mut [u8]) {
    u8x64::simd_chunks(buff, |data| {
        data ^ 0x15
    });
}