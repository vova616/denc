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
pub fn encrypt(buf: &mut [u8]) {
    buf.iter_mut().zip(KEY.iter().cycle()).for_each(|(data, &k)| {
        if !(*data == 0 || *data == k) {
            *data ^= k;
        }
    });
}
