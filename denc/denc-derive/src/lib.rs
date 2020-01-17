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

    let decoder_decode_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            println!("a {}", const_size);
            decoder.ensure(const_size);
            decoder.ensure(<Dec as Decode<#ty>>::size(decoder));
            let #name = <Dec as Decode<#ty>>::decode(decoder);
            const_size -= <Dec as Decode<#ty>>::SIZE;
            println!("b {}", const_size);
        }
    });
    let decoder_size_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            + <Dec as Decode<#ty>>::SIZE
        }
    });
    let decoder_size_impl2 = input.fields.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            + <Dec as Decode<#ty>>::SIZE
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
        impl<#ty_generics Dec: Decoder> Decode<#name #ty_generics #where_clause> for Dec {
            const SIZE: usize = 0 #(
                #decoder_size_impl
             )*;

            #[inline]
            fn decode(decoder: &mut Dec) -> #name #ty_generics #where_clause {
                let mut const_size = 0 #(
                    #decoder_size_impl2
                )*;

                #(
                    #decoder_decode_impl
                )*

                #name {
                    #(
                        #decoder_decode_return_impl,
                    )*
                }
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
