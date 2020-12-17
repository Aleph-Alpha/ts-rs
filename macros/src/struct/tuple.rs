use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsUnnamed, ItemStruct, Result};

use crate::attr::{FieldAttr, StructAttr};
use crate::DerivedTS;

pub(crate) fn tuple(s: &ItemStruct, i: &FieldsUnnamed) -> Result<DerivedTS> {
    let StructAttr { rename_all, rename } = StructAttr::from_attrs(&s.attrs)?;
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to tuple structs");
    }

    let name = rename.unwrap_or_else(|| s.ident.to_string());
    let fields = i
        .unnamed
        .iter()
        .map(format_field)
        .flat_map(|x| match x {
            Ok(Some(x)) => Some(Ok(x)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    Ok(DerivedTS {
        format: quote!(format!("[{}]", vec![#(#fields),*].join(", "))),
        decl: quote!(format!("export type {} = {};", #name, Self::format(0, true))),
        flatten: None,
        name,
    })
}

fn format_field(field: &Field) -> Result<Option<TokenStream>> {
    let ty = &field.ty;
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip,
        flatten,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(None);
    }

    if rename.is_some() {
        syn_err!("`rename` is not applicable to tuple structs")
    }
    if flatten {
        syn_err!("`flatten` is not applicable to newtype fields")
    }

    Ok(Some(match type_override {
        Some(o) => quote!(#o.into()),
        None => quote!(<#ty as ts_rs::TS>::format(0, #inline)),
    }))
}
