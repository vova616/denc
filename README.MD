# Denc
![](https://github.com/vova616/denc/workflows/Rust/badge.svg)

Serialization library like serde but easier to customize and ultimatly should be faster.
the library is not ready to use yet, its using unstable nightly features and changes a lot.

Features:
* Vec/Const Arrays
* Custom types
* decode -> T and decode_into(&mut T) -> (), decode_into is usually faster than decode
* no need for default

Unsafety:
* there is some, mostly used for creating arrays without default

# Example

```
#[derive(Denc, Enc)]
pub struct TestStructTiny {
    pub a: u16,
    pub b: u8,
}


fn main() {
    let a: TestStructTiny = LittleEndian::from_slice(&[1u8, 0, 2]).unwrap();
    let a: TestStructTiny = LittleEndian::from_reader(&[1u8, 0, 2] as &[u8]).unwrap();
    assert_eq!(a.a, 1);
    assert_eq!(a.b, 2);
}
```

#Example decoding impl

```
impl<'a> Decode<LittleEndian<'a>> for u32 {
    const SIZE: usize = 4;

    #[inline(always)]
    fn decode(decoder: &mut LittleEndian<'a>) -> Result<Self, &'static str> {
        let slice = decoder.buff_advance_exact(4).ok_or(EOF)?;
        Ok(u32::from_le_bytes(slice.try_into().ok().ok_or(EOF)?))
    }
}

```
