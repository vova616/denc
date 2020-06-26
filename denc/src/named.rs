use crate::{Decode, Decoder, Encode, Encoder, LittleEndian};
use std::ops::Index;

pub trait NamedDecoder: Decoder {
    fn next_identifier(&mut self) -> Result<Option<usize>, Self::Error>;
}

pub trait Dic {
    const FIELDS: &'static [&'static str];

    
    fn from_index(index: usize) -> usize {
        index + 1
    }

    fn from_bytes(bytes: &[u8]) -> usize {
        for field in Self::FIELDS.iter().map(|s| s.as_bytes()).enumerate() {
            if field.1 == bytes {
                return field.0 + 1
            }
        }
        return 0
    }

    fn from_str(string: &str) -> usize {
        Self::from_bytes(string.as_bytes())
    }
}

pub trait NamedEncoder: Encoder {
    fn encode_named<T: Encode<Self>>(
        &mut self,
        name: &'static str,
        id: usize,
        value: &T,
    ) -> Result<usize, Self::Error>;
}

impl<'a> NamedDecoder for LittleEndian<'a> {
    fn next_identifier(&mut self) -> Result<Option<usize>, Self::Error> {
        if self.0.len() <= 0 {
            return Ok(None);
        }
        Ok(Some(self.0[0] as usize + 1))
    }
}

pub struct TestStruct {
    pub a: u16,
    pub b: u32,
}

impl Dic for TestStruct {
    const FIELDS: &'static [&'static str] = &["a", "b"];
}

//basically this and use 2 impl, one with default and the other is NamedDecoder

impl<Dec: NamedDecoder> Decode<Dec> for TestStruct
where
    u32: Decode<Dec>,
    u16: Decode<Dec>,
{
    const SIZE: usize = <u16 as Decode<Dec>>::SIZE + <u32 as Decode<Dec>>::SIZE;
    #[inline(always)]
    fn decode<'b>(decoder: &'b mut Dec) -> Result<Self, Dec::Error> {
        let mut a: Option<u16> = None;
        let mut b: Option<u32> = None;

 
        loop {
            let id = decoder.next_identifier()?;
            match id {
                None => break,
                Some(id) => {
                    match id {
                        1 => a = Some(decoder.decode()?),
                        2 => b = Some(decoder.decode()?),
                        _ => {}
                    };
                }
            }
        }
        Ok(TestStruct {
            a: a.unwrap(),
            b: b.unwrap(),
        })
    }
    #[inline(always)]
    fn decode_into(decoder: &mut Dec, value: &mut Self) -> Result<(), Dec::Error> {
        <u16 as Decode<Dec>>::decode_into(decoder, &mut value.a)?;
        <u32 as Decode<Dec>>::decode_into(decoder, &mut value.b)?;
        Ok(())
    }
}

/*
impl<T: NamedDecoder> Named<T> for TestStruct
where
    u16: Decode<T>,
    u32: Decode<T>,
{
    fn decode_named(decoder: &mut T) -> Result<Self, T::Error> {
        let mut a: Option<u16> = None;
        let mut b: Option<u32> = None;

        loop {
            let id = decoder.next_identifier()?;
            match id {
                None => break,
                Some(slice) => {
                    let index = Self::name_to_id(slice);
                    match index {
                        1 => a = Some(decoder.decode()?),
                        2 => b = Some(decoder.decode()?),
                        _ => {}
                    };
                }
            }
        }
        Ok(TestStruct {
            a: a.unwrap(),
            b: b.unwrap(),
        })
    }

    fn name_to_id(name: &[u8]) -> u16 {
        match name {
            b"a" => 1,
            b"b" => 2,
            _ => 0,
        }
    }
    fn decode_into_named(&mut self, decoder: &mut T) -> Result<(), T::Error> {
        Ok(*self = Self::decode_named(decoder)?)
    }
}
*/
