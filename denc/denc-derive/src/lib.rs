#![feature(generators, generator_trait)]
#![recursion_limit = "128"]

extern crate proc_macro;

use self::proc_macro::TokenStream;
use quote::quote;
use syn;

extern crate proc_macro2;
use self::proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(MapperDec)]
pub fn derive_mapper_dec(input: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(input).unwrap();

    let types: Vec<syn::Type> = input
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ty.clone())
        .collect();
    let decoder_decode_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        let concatenated = format!("_dec_{}", name);
        let size_name = syn::Ident::new(&concatenated, name.span());
        let inner_types = types.iter().skip(i);
       
        if i == 0 {
            quote! {
                if decoder.len() < <#ty as Decode<Dec>>::SIZE {
                    return Err(Dec::EOF);
                }
                let #name = <#ty as Decode<Dec>>::decode(decoder)?;
            }
        } else {
            let prev_type = types.get(i - 1).unwrap();
            quote! {
                if !<#prev_type as Decode<Dec>>::STATIC && decoder.len() < <#ty as Decode<Dec>>::SIZE {
                    return Err(Dec::EOF);
                }
                let #name = <#ty as Decode<Dec>>::decode(decoder)?;
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
        impl<#ty_generics Dec: Decoder> Decode<Dec> for #name #ty_generics #where_clause {
            const SIZE: usize = #(
                <#types as Decode<Dec>>::SIZE
             )+*;

            #[inline(always)]
            fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error>  {
                //decoder.fill_buffer(<#name as Decode<Dec>>::SIZE);

                #(
                    #decoder_decode_impl
                )*

                Ok(#name {
                    #(
                        #decoder_decode_return_impl,
                    )*
                })
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
