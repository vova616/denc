#![feature(generators, generator_trait)]
#![recursion_limit="128"]

extern crate proc_macro;

use proc_macro2;
use self::proc_macro::TokenStream;
use self::proc_macro2::{Ident, Span};
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
                    <#ty>::size(&buffer[index..])
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
                   for item in &mut #name {
                       let _s_size = <#ty>::size(&_s_buffer);
                       let #name = <#ty>::decode(&_s_buffer[.._s_size]);
                       _s_buffer = &_s_buffer[_s_size..];
                   }
                }
            }
            _ => {
                quote! {
                   let _s_size = <#ty>::size(&_s_buffer);
                   let #name = <#ty>::decode(&_s_buffer[.._s_size]);
                   _s_buffer = &_s_buffer[_s_size..];
                }
            }
        }

    });
    let decoder_decode_return_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        quote! {
           #name: #name
        }
    });
    let name = &input.ident;
    let attrs = &input.attrs;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl<'a> Decoder<'a> for #name #ty_generics #where_clause {
            type Output = Self;

            fn decode(mut _s_buffer: &'a [u8]) -> Self {
                #(
                   #decoder_decode_impl
                )*
                Self{
                    #(
                        #decoder_decode_return_impl,
                    )*
                }
            }

            fn size(buffer: &'a [u8]) -> usize {
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
                 self.#name.iter().fold(0, |sum, x| sum + x.size_enc()) as usize
                }
            },
            _ => {
                quote! {
                     self.#name.size_enc()
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
                    for item in &self.#name {
                        item.encode_into(buff);
                    }
                }
            }
            _ => {
                quote! {
                     self.#name.encode_into(buff);
                }
            }
        }
    });
    let name = &input.ident;
    let attrs = &input.attrs;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl #impl_generics Encoder for #name #ty_generics #where_clause {
             fn encode_into<T : std::io::prelude::Write>(&self, buff: &mut T) {
                #(
                   #decoder_decode_impl
                )*
            }

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