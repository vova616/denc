use crate::{Decoder, Encoder};
use smallvec::SmallVec;
use std::convert::{TryFrom, TryInto};
use std::io::prelude::Write;
use std::marker::PhantomData;
use std::mem;

pub struct List<T, S> {
    pub items: SmallVec<[T; 8]>,
    phantom2: PhantomData<S>,
}

impl<'b, T: Decoder<'b, Output = T> + 'b, S> List<T, S> {
    #[inline(always)]
    fn new(mut buffer: &'b [u8], len: usize) -> Self {
        let mut new_list = Self::with_capacity(len);
        unsafe {
            new_list.items.set_len(len);
        }
        new_list.items.iter_mut().for_each(|item| {
            let size = T::size(buffer);
            *item = T::decode(&buffer[..size]);
            buffer = &buffer[size..];
        });
        new_list
    }

    #[inline(always)]
    fn with_capacity(cap: usize) -> Self {
        List {
            items: SmallVec::with_capacity(cap),
            phantom2: PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    #[inline(always)]
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    #[inline(always)]
    pub fn vec(&self) -> &SmallVec<[T; 8]> {
        &self.items
    }
}

impl<T, S> From<Vec<T>> for List<T, S> {
    #[inline(always)]
    fn from(vec: Vec<T>) -> Self {
        List {
            items: SmallVec::from(vec),
            phantom2: PhantomData,
        }
    }
}

impl<T: Encoder, S: Encoder + Default> Encoder for List<T, S>
where
    S: TryFrom<usize>,
{
    #[inline(always)]
    fn encode_into<G: Write>(&self, buff: &mut G) {
        let len = match self.items.len().try_into() {
            Ok(s) => s,
            _ => panic!("Cannot convert to usize"),
        };
        S::encode_into(&len, buff);
        self.items.iter().for_each(|item| item.encode_into(buff));
    }

    #[inline(always)]
    fn size_enc(&self) -> usize {
        let len = match self.items.len().try_into() {
            Ok(s) => s,
            _ => panic!("Cannot convert to usize"),
        };
        self.items
            .iter()
            .fold(S::size_enc(&len), |sum, x| sum + x.size_enc()) as usize
    }
}

impl<'b, T: Decoder<'b, Output = T> + 'b, S: Decoder<'b, Output = S> + Default> Decoder<'b>
    for List<T, S>
where
    S::Output: Into<usize>,
{
    type Output = List<T, S>;

    #[inline(always)]
    fn decode(buffer: &'b [u8]) -> Self::Output {
        let size = S::size(buffer);
        let len = S::decode(buffer).into();
        Self::Output::new(&buffer[size..], len)
    }

    #[inline(always)]
    fn size(buffer: &'b [u8]) -> usize {
        let len = S::decode(buffer).into();
        (0..len)
            .into_iter()
            .fold(S::size(buffer), |sum, x| sum + T::size(&buffer[sum..])) as usize
    }
}

pub struct RefList<'b, T, S> {
    pub buffer: &'b [u8],
    pub len: S,
    phantom: PhantomData<T>,
    phantom2: PhantomData<S>,
}

impl<'b, T, S: Clone> RefList<'b, T, S> {
    #[inline(always)]
    fn new(buffer: &'b [u8], len: S) -> Self {
        RefList {
            buffer: buffer,
            len: len,
            phantom: PhantomData,
            phantom2: PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> RefList<'b, T, S> {
        RefList {
            buffer: self.buffer,
            len: self.len.clone(),
            phantom: self.phantom,
            phantom2: self.phantom2,
        }
    }
}

impl<'b, T: Encoder + 'b, S: Encoder + Default> Encoder for RefList<'b, T, S> {
    #[inline(always)]
    fn encode_into<G: Write>(&self, buff: &mut G) {
        S::encode_into(&self.len, buff);
        buff.write(self.buffer);
    }

    #[inline(always)]
    fn size_enc(&self) -> usize {
        self.buffer.len()
    }
}

impl<'b, T: Decoder<'b> + 'b, S: Decoder<'b, Output = S> + Default + Clone> Decoder<'b>
    for RefList<'b, T, S>
where
    S::Output: Into<usize>,
{
    type Output = RefList<'b, T, S>;

    #[inline(always)]
    fn decode(buffer: &'b [u8]) -> Self::Output {
        let size = S::size(buffer);
        let len = S::decode(buffer);
        Self::Output::new(&buffer[size..], len)
    }

    #[inline(always)]
    fn size(buffer: &'b [u8]) -> usize {
        let len = S::decode(buffer).into();
        (0..len)
            .into_iter()
            .fold(S::size(buffer), |sum, x| sum + T::size(&buffer[sum..])) as usize
    }
}

impl<'b, T: Decoder<'b> + 'b, S> Iterator for RefList<'b, T, S> {
    type Item = T::Output;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.len() > 0 {
            let size = T::size(self.buffer);
            let b = T::decode(&self.buffer[..size]);
            self.buffer = &self.buffer[size..];
            Some(b)
        } else {
            None
        }
    }
}
