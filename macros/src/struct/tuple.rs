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
        .flat_map(Result::transpose)
        .collect::<Result<Vec<TokenStream>>>()?;

    Ok(DerivedTS {
        inline: quote!{
            format!(
                "[{}]", 
                vec![#(#fields),*].join(", ")
            )
        },
        decl: quote!{
            format!(
                "export type {} = {};", 
                #name, 
                Self::inline(0)
            )
        },
        inline_flattened: None,
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
        Some(o) => quote!(#o.to_owned()),
        None if inline => quote!(<#ty as ts_rs::TS>::inline(0)),
        None => quote!(<#ty as ts_rs::TS>::name()),
    }))
}
