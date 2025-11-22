use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Fields, parse_macro_input, spanned::Spanned};
use wasm_actions_parse::{InputAttr, OutputAttr, ParseFieldsNamed};

use crate::{
    codegen::{InputField, OutputField},
};
mod codegen;

#[proc_macro_derive(ActionInput, attributes(input))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match input.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let fields = match InputAttr::parse_fields_named(fields) {
                Ok(f) => f.into_iter().map(|e| {
                   InputField {
                    span: e.span,
                    field: e.ident,
                    attrs: e.attrs,
                } 
                }).collect(),
                Err(e) => return compile_error(e).into()
            };
            let struct_name = input.ident;
            codegen::action_input_impl(struct_name, fields)
                .unwrap_or_else(compile_error)
                .into()
        }
        _ => compile_error(Error::new(
            input.span(),
            "`#[derive(ActionInput)]` only supports non-tuple structs",
        ))
        .into(),
    }
}

#[proc_macro_derive(ActionOutput, attributes(output))]
pub fn derive_output(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match input.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let fields = match OutputAttr::parse_fields_named(fields) {
                Ok(f) => f.into_iter().map(|e| {
                   OutputField {
                    field: e.ident,
                    attrs: e.attrs,
                } 
                }).collect(),
                Err(e) => return compile_error(e).into()
            };
            let struct_name = input.ident;
            codegen::action_output_impl(struct_name, fields)
                .unwrap_or_else(compile_error)
                .into()
        }
        _ => compile_error(Error::new(
            input.span(),
            "`#[derive(ActionOutput)]` only supports non-tuple structs",
        ))
        .into(),
    }
}

#[proc_macro_attribute]
pub fn wasm_action(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();
    let start_fn = codegen::start_fn(parse_macro_input!(input)).unwrap_or_else(compile_error);
    let start_fn: TokenStream = start_fn.into();
    output.extend(start_fn);
    output
}

fn compile_error(error: syn::Error) -> proc_macro2::TokenStream {
    let err = error.to_compile_error();
    quote!(
        #err
    )
}
