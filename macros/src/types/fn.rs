use quote::ToTokens;
use syn::{ItemFn, Result, Error, PatType, spanned::Spanned, punctuated::Punctuated, token::{Comma, Paren, Brace}, FieldsUnnamed, FieldsNamed, Fields};

use crate::{attr::{FnAttr, Args}, DerivedTS};

pub fn fn_def(input: &ItemFn, fn_attr: FnAttr) -> Result<DerivedTS> {
    let fields = input
        .sig
        .inputs
        .iter()
        .map(|x| match x {
            syn::FnArg::Receiver(_) => Err(
                Error::new(x.span(), "self parameter is not allowed")
            ),
            syn::FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                Ok(syn::Field {
                    attrs: attrs.to_vec(),
                    vis: syn::Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: match fn_attr.args {
                        Args::Named => Some(syn::parse2(pat.to_token_stream())?),
                        Args::Positional => None,
                    },
                    colon_token: None,
                    ty: *ty.clone(),
                })
            },
        })
        .collect::<Result<Punctuated<_, Comma>>>()?;

    let fields = match (fields.is_empty(), fn_attr.args) {
        (true, _) => Fields::Unit,
        (_, Args::Positional) => Fields::Unnamed(FieldsUnnamed {
            paren_token: Paren::default(),
            unnamed: fields,
        }),
        (_, Args::Named) => Fields::Named(FieldsNamed {
            brace_token: Brace::default(),
            named: fields,
        }),
    };

    Ok(DerivedTS {
        ..todo!()
    })
}
