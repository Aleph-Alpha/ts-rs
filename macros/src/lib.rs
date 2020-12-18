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
    inline: TokenStream,
    decl: TokenStream,
    inline_flattened: Option<TokenStream>,
}

impl DerivedTS {
    fn into_impl(self, rust_ty: Ident) -> TokenStream {
        let DerivedTS {
            name,
            inline,
            decl,
            inline_flattened,
        } = self;
        let inline_flattened = inline_flattened
            .map(|t| {
                quote! {
                    fn inline_flattened(indent: usize) -> String {
                        #t
                    }
                }
            })
            .unwrap_or_else(TokenStream::new);

        quote! {
            impl ts_rs::TS for #rust_ty {
                fn decl() -> String {
                    #decl
                }
                fn name() -> String {
                    #name.to_owned()
                }
                fn inline(indent: usize) -> String {
                    #inline 
                }
                #inline_flattened
            }
        }
    }
}

/// Derives [TS](TS) for a struct or enum.
/// Please take a look at [TS](TS) for documentation.
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
