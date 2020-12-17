#![macro_use]
#![deny(unused)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Item, Result};

#[macro_use]
mod utils;
mod attr;
mod r#enum;
mod r#struct;

struct DerivedTS {
    name: String,
    format: TokenStream,
    decl: TokenStream,
    flatten: Option<TokenStream>,
}

impl DerivedTS {
    fn into_impl(self, rust_ty: Ident) -> TokenStream {
        let DerivedTS {
            name,
            format,
            decl,
            flatten,
        } = self;
        let flatten = flatten.unwrap_or_else(|| quote!(panic!("this type cannot be flattened")));

        quote! {
            impl ts_rs::TS for #rust_ty {
                fn decl() -> Option<String> { Some({#decl}) }
                fn format(indent: usize, inline: bool) -> String {
                    match inline {
                        true => { #format },
                        false => #name.into()
                    }
                }
                fn flatten_interface(indent: usize) -> String { #flatten }
            }
        }
    }
}

/// Derives [TS](ts_rs::TS) for a struct or enum.
/// Please take a look at [TS](ts_rs::TS) for documentation.
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
