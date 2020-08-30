#![feature(generators, generator_trait)]
#![recursion_limit = "128"]

extern crate proc_macro;

use self::proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_quote;

extern crate proc_macro2;
use self::proc_macro2::TokenStream as TokenStream2;
use std::collections::HashSet;
use syn::parse::Parse;
use syn::{Generics, ImplGenerics};

#[proc_macro_derive(Denc)]
pub fn derive_mapper_dec(input: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(input).unwrap();

    let mut types: Vec<syn::Type> = input
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ty.clone())
        .collect();

    let set: HashSet<syn::Type> = types.iter().map(|t| t.clone()).collect(); // dedup
    let types_uniq = set.into_iter();

    //let types_uniq = types.so
    let decoder_decode_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #name: <#ty as denc::Decode<Dec>>::decode(decoder)?,
        }
    });
    let decoder_decode_impl2 = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            <#ty as denc::Decode<Dec>>::decode_into(decoder, &mut value.#name)?;
        }
    });
    let name = &input.ident;
    let attrs = &input.attrs;

    let mut generics_clone = input.generics.clone();
    generics_clone
        .params
        .push(parse_quote! { Dec: denc::Decoder });
    let (impl_generics, _, _) = generics_clone.split_for_impl();

    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl #impl_generics denc::Decode<Dec> for #name #ty_generics
            where
            #(
                #types_uniq : denc::Decode<Dec>
            ),*

        {
            const SIZE: usize = #(
                <#types as denc::Decode<Dec>>::SIZE
             )+*;

            #[inline(always)]
            default fn decode<'b>(decoder: &'b mut Dec) -> Result<Self, Dec::Error> {
                Ok(#name {
                    #(
                        #decoder_decode_impl
                    )*
                })
            }

            #[inline(always)]
            default fn decode_into(decoder: &mut Dec, value: &mut Self) -> Result<(), Dec::Error> {
                #(
                    #decoder_decode_impl2
                )*
                Ok(())
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(Enc)]
pub fn derive_mapper_enc(input: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(input).unwrap();

    let mut types: Vec<syn::Type> = input
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ty.clone())
        .collect();

    let set: HashSet<syn::Type> = types.iter().map(|t| t.clone()).collect(); // dedup
    let types_uniq = set.into_iter();

    //let types_uniq = types.so
    let decoder_decode_impl = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        let concatenated = format!("_enc_{}", name);
        let size_name = syn::Ident::new(&concatenated, name.span());
        let inner_types = types.iter().skip(i);

        if i == 0 {
            quote! {
                <#ty as denc::Encode<Enc>>::encode(&self.#name, encoder)?;
            }
        } else {
            let prev_type = types.get(i - 1).unwrap();
            quote! {
                <#ty as denc::Encode<Enc>>::encode(&self.#name, encoder)?;
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

    let mut generics_clone = input.generics.clone();
    generics_clone
        .params
        .push(parse_quote! { Enc: denc::Encoder });
    let (impl_generics, _, _) = generics_clone.split_for_impl();

    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl #impl_generics denc::Encode<Enc> for #name #ty_generics
            where
            #(
                #types_uniq : denc::Encode<Enc>
            ),*

        {
            const SIZE: usize = #(
                <#types as denc::Encode<Enc>>::SIZE
             )+*;

            #[inline(always)]
            fn encode(&self, encoder: &mut Enc) -> Result<(), Enc::Error>  {
                //decoder.fill_buffer(<#name as Decode<Dec>>::SIZE);

                #(
                    #decoder_decode_impl
                )*

                Ok(())
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(Dic)]
pub fn derive_mapper_dic(input: TokenStream) -> TokenStream {
    let mut input: syn::ItemStruct = syn::parse(input).unwrap();

    let mut types: Vec<syn::Type> = input
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.ty.clone())
        .collect();

    let set: HashSet<syn::Type> = types.iter().map(|t| t.clone()).collect(); // dedup
    let types_uniq = set.into_iter();

    //let types_uniq = types.so
    let lets_fields = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            let mut #name: Option<#ty> = None;

        }
    });
    let struct_fields = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #name: #name.unwrap()
        }
    });
    let name_ids: Vec<_> = input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let iplus1 = (i + 1) as u16;
        quote! {
            std::stringify!(#name) => #iplus1
        }
    }).collect();
    let decode_impl= input.fields.iter().enumerate().map(|(i, f)| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let iplus1 = i + 1;
        quote! {
           #iplus1 => #name = Some(decoder.decode()?)
        }
    });
    let name = &input.ident;
    let attrs = &input.attrs;

    let mut generics_clone = input.generics.clone();
    generics_clone
        .params
        .push(parse_quote! { Dec: denc::NamedDecoder });
    let (impl_generics, _, _) = generics_clone.split_for_impl();

    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl #impl_generics Decode<Dec> for #name #ty_generics
            where
            #(
                #types_uniq : denc::Decode<Dec>
            ),*

        {
            #[inline(always)]
            fn decode(decoder: &mut Dec) -> Result<Self, Dec::Error> {
                #(
                    #lets_fields
                )*

                loop {
                    let id = decoder.next_identifier()?;
                    match id {
                        None => break,
                        Some(id) => {
                            match id {
                                #(
                                    #decode_impl,
                                )*
                                _ => {}
                            };
                        }
                    }
                }

                Ok(#name {
                    #(
                        #struct_fields
                    ),*
                })
            }
        }
    };

    TokenStream::from(output)
}
