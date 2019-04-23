use packed_simd::{u8x16, u8x32, shuffle};
use packed_simd::FromCast;
use packed_simd::Cast;
use packed_simd::{FromBits, IntoBits};

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
pub fn decrypt2(buff: &mut [u8]) {
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
pub fn decrypt3(buff: &mut [u8]) {
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
fn decrypt_index(buf: &mut [u8], skip_key: usize) {
    buf.iter_mut().zip(KEY.iter().cycle().skip(skip_key)).for_each(|(data, &k)| {
        if !(*data == 0 || *data == k) {
            *data ^= k;
        }
    });
}

#[inline(always)]
pub fn decrypt_hybrid2(buff: &mut [u8]) {
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
    let index = buff.len()-buff.len()%32;
    let range = index..;
    decrypt_index(&mut buff[range], index);
}

#[inline(always)]
pub fn decrypt_hybrid3(buff: &mut [u8]) {
    let mut key =  u8x16::new(0x71,0x6d,0x66,0x61,0x6b,0x74,0x6e,0x70,0x67,0x6a,0x73,0x71,0x6d,0x66,0x61,0x6b);
    let zero = u8x16::splat(0);
    for mut slice in buff.chunks_exact_mut(u8x16::lanes()) {
        let x = u8x16::from_slice_unaligned(slice);
        let y = key.eq(x) | zero.eq(x);
        let b = y.select(x, x ^ key);
        b.write_to_slice_unaligned(&mut slice);
        key = shuffle!(key, [5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9])
    }
    let index = buff.len()-buff.len()%16;
    let range = index..;
    decrypt_index(&mut buff[range], index);
}

#[inline(always)]
pub fn decrypt_hybrid(buff: &mut [u8]) {
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

    let index = buff.len()-buff.len()%32;
    let range = index..;
    buff[range].iter_mut().zip(0usize..32usize).for_each(|(data, k)| {
        let kk: u8 = key.extract(k as usize);
        if !(*data == 0 || *data == kk) {
            *data ^= kk;
        }
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
