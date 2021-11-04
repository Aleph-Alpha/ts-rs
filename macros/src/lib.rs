#![macro_use]
#![deny(unused)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_quote, spanned::Spanned, GenericParam, Generics, Item, Result, WhereClause};

use crate::deps::Dependencies;

#[macro_use]
mod utils;
mod attr;
mod deps;
mod types;

struct DerivedTS {
    name: String,
    inline: TokenStream,
    decl: TokenStream,
    inline_flattened: Option<TokenStream>,
    dependencies: Dependencies,
}

impl DerivedTS {
    fn into_impl(self, rust_ty: Ident, generics: Generics) -> TokenStream {
        let DerivedTS {
            name,
            inline,
            decl,
            inline_flattened,
            dependencies,
            ..
        } = self;
        let inline_flattened = inline_flattened
            .map(|t| {
                quote! {
                    fn inline_flattened() -> String {
                        #t
                    }
                }
            })
            .unwrap_or_else(TokenStream::new);

        let Generics {
            ref lt_token,
            ref params,
            ref gt_token,
            where_clause: _,
        } = generics;

        let where_clause = add_ts_trait_bound(&generics);

        quote! {
            impl #lt_token #params #gt_token ts_rs::TS for #rust_ty #lt_token #params #gt_token #where_clause {
                fn decl() -> String {
                    #decl
                }
                fn name() -> String {
                    #name.to_owned()
                }
                fn inline() -> String {
                    #inline
                }
                #inline_flattened
                fn dependencies() -> Vec<(std::any::TypeId, String)> {
                    #dependencies
                }
                fn transparent() -> bool {
                    false
                }
            }
        }
    }
}

fn add_ts_trait_bound(generics: &Generics) -> Option<WhereClause> {
    let generic_types: Vec<_> = generics
        .params
        .iter()
        .filter_map(|gp| match gp {
            GenericParam::Type(ty) => Some(ty.ident.clone()),
            _ => None,
        })
        .collect();
    if generic_types.is_empty() {
        return generics.where_clause.clone();
    }
    match generics.where_clause {
        None => Some(parse_quote! { where #( #generic_types : ts_rs::TS ),* }),
        Some(ref w) => {
            let bounds = w.predicates.iter();
            Some(parse_quote! { where #(#bounds,)* #( #generic_types : ts_rs::TS ),* })
        }
    }
}

/// Derives [TS](./trait.TS.html) for a struct or enum.
/// Please take a look at [TS](./trait.TS.html) for documentation.
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
    let (ts, ident, generics) = match input {
        Item::Struct(s) => (types::struct_def(&s)?, s.ident, s.generics),
        Item::Enum(e) => (types::r#enum(&e)?, e.ident, e.generics),
        _ => syn_err!(input.span(); "unsupported item"),
    };

    Ok(ts.into_impl(ident, generics))
}
