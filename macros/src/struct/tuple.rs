use proc_macro2::TokenStream;
use quote::quote;
use syn::{FieldsUnnamed, ItemStruct, Result};

use crate::attr::FieldAttr;
use crate::DerivedTS;

pub(crate) fn tuple(s: &ItemStruct, i: &FieldsUnnamed) -> Result<DerivedTS> {
    let name = s.ident.to_string();
    let fields = i
        .unnamed
        .iter()
        .map(|field| {
            let ty = &field.ty;
            let FieldAttr {
                type_override,
                rename,
                inline,
            } = FieldAttr::from_attrs(&field.attrs)?;

            if rename.is_some() {
                syn_err!("`rename` is not applicable to tuple structs")
            }

            Ok(match type_override {
                Some(o) => quote!(#o.into()),
                None => quote!(<#ty as ts_rs::TS>::format(0, #inline)),
            })
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    Ok(DerivedTS {
        format: quote!(format!("[{}]", vec![#(#fields),*].join(", "))),
        decl: quote!(format!("export type {} = {};", #name, Self::format(0, true))),
        name,
    })
}
