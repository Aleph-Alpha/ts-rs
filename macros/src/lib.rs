#![macro_use]
#![deny(unused)]

//extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Item, Result};

#[macro_use]
mod macros;
mod attr;
mod r#enum;
mod r#struct;

struct DerivedTS {
    name: String,
    format: TokenStream,
    decl: TokenStream,
}

impl DerivedTS {
    fn into_impl(self, rust_ty: Ident) -> TokenStream {
        let DerivedTS { name, format, decl } = self;
        quote! {
            impl ts_rs::TS for #rust_ty {
                fn decl() -> Option<String> { #decl.into() }
                fn format(indent: usize, inline: bool) -> String {
                    match inline {
                        true => #format,
                        false => #name.into()
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(TS, attributes(ts))]
pub fn typescript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match entry(input) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}

fn entry(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let input = syn::parse::<Item>(input)?;
    let (ts, ident) = match input {
        Item::Struct(s) => (r#struct::struct_def(&s)?, s.ident),
        Item::Enum(e) => (r#enum::r#enum(&e)?, e.ident),
        _ => syn_err!(input.span(); "unsupported item"),
    };

    Ok(ts.into_impl(ident))
}
