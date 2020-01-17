#![feature(generators, generator_trait)]
#![recursion_limit = "128"]

extern crate proc_macro;

use self::proc_macro::TokenStream;
use self::proc_macro2::{Ident, Span};
use proc_macro2;
use quote::quote;
use syn;

#[proc_macro_derive(MapperDec)]
pub fn derive_mapper_dec(input: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(input).unwrap();

    let decoder_store_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;
        match ty {
            syn::Type::Array(array) => {
                let ty = &array.elem;
                let len = &array.len;
                quote! {
                   (0..#len).into_iter().fold(0, |sum, x| sum + <#ty>::size(&buffer[sum..])) as usize
                }
            }
            _ => {
                quote! {
                    <#ty as Decode<'a, Decr>>::size(&buffer[index..])
                }
            }
        }
    });
    let decoder_decode_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;
        match ty {
            syn::Type::Array(array) => {
                let ty = &array.elem;
                let len = &array.len;
                quote! {
                   let mut #name: [#ty; #len];
                   unsafe {
                       #name = std::mem::uninitialized();
                   }
                   let _s_target = decoder;
                   for item in &mut #name {
                       _s_buffer = &_s_buffer[_s_size..];
                       _s_size = <<#ty> as Decoder>::size(&_s_buffer);
                       let #name = <#ty>::decode(&_s_buffer[.._s_size]);
                   }
                }
            }
            _ => {
                quote! {
                   let _s_size = <#ty as Decode<'a, Decr>>::size(decoder);
                   let (_s_target, decoder) = decoder.split_at(_s_size);
                   let #name = <#ty as Decode<'a, Decr>>::decode(_s_target);
                }
            }
        }
    });
    let decoder_decode_return_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        quote! {
           #name
        }
    });
    let name = &input.ident;
    let attrs = &input.attrs;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl<'a, Data, Decr: Decoder<'a, Data = Data>> Decode<'a, Decr> for #name #ty_generics #where_clause {
            #[inline]
            fn decode(decoder: &'a [Data]) -> Self {
                #(
                   #decoder_decode_impl
                )*
                Self{
                    #(
                        #decoder_decode_return_impl,
                    )*
                }
            }

            #[inline]
            fn size(buffer: &'a [Data]) -> usize {
                let mut index = 0;
                #(
                   index += #decoder_store_impl;
                )*
                index
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(MapperEnc)]
pub fn derive_mapper_enc(input: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(input).unwrap();

    let decoder_store_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;

        match ty {
            syn::Type::Array(array) => {
                let ty = &array.elem;
                let len = &array.len;
                quote! {
                 self.#name.iter().fold(0, |sum, x| sum + Encode::size(&x) as usize)
                }
            }
            _ => {
                quote! {
                     self.#name.size_enc()
                }
            }
        }
    });
    let encoder_encode_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;
        match ty {
            syn::Type::Array(array) => {
                let ty = &array.elem;
                let len = &array.len;
                quote! {
                    for item in &self.#name {
                        item.encode(buff);
                    }
                }
            }
            _ => {
                quote! {
                    let a_size = <#ty as Encode<'b, Encdr>>::size(&self.#name);
                    let (target, encoder) = encoder.split_at_mut(a_size);
                    let #name = <#ty as Encode<'b, Encdr>>::encode(&self.#name, target);
                    self.#name.encode(buff);
                }
            }
        }
    });
    let name = &input.ident;
    let attrs = &input.attrs;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        //impl #impl_generics Encoder for #name #ty_generics #where_clause {
        impl<'a, Data, Encdr: Encoder<'a, Data = Data>>  Encode<'a, Encdr> for #name #ty_generics #where_clause {

            #[inline]
             fn encode(&self, buff: &'a mut [Data]) {
                #(
                   #encoder_encode_impl
                )*
            }

            #[inline]
            fn size_enc(&self) -> usize {
                let mut size = 0;
                #(
                   size += #decoder_store_impl;
                )*
                size
            }
        }
    };

    TokenStream::from(output)
}
