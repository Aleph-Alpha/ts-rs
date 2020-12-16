use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsNamed, ItemStruct, Result};

use crate::attr::{FieldAttr, Inflection, StructAttr};
use crate::DerivedTS;

pub(crate) fn named(s: &ItemStruct, i: &FieldsNamed) -> Result<DerivedTS> {
    let StructAttr { rename_all, rename } = StructAttr::from_attrs(&s.attrs)?;
    let name = rename.unwrap_or_else(|| s.ident.to_string());
    let fields = i
        .named
        .iter()
        .map(|f| format_field(f, &rename_all))
        .flat_map(|x| match x {
            Ok(Some(x)) => Some(Ok(x)),
            Ok(None) => None,
            Err(err) => Some(Err(err))
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    Ok(DerivedTS {
        format: quote! {
            format!(
                "{{\n{}\n{}}}",
                vec![#(#fields),*].join("\n"),
                " ".repeat(indent * 4)
            )
        },
        decl: quote! {
            format!("export interface {} {}", #name, Self::format(0, true))
        },
        name,
    })
}

fn format_field(field: &Field, rename_all: &Option<Inflection>) -> Result<Option<TokenStream>> {
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip
    } = FieldAttr::from_attrs(&field.attrs)?;
    
    if skip {
        return Ok(None)
    }

    let ty = &field.ty;
    let ty = type_override
        .map(|t| quote!(#t))
        .unwrap_or_else(|| quote!(<#ty as ts_rs::TS>::format(indent + 1, #inline)));
    let name = match (rename, rename_all) {
        (Some(rn), _) => rn,
        (None, Some(rn)) => rn.apply(&field.ident.as_ref().unwrap().to_string()),
        (None, None) => field.ident.as_ref().unwrap().to_string(),
    };

    Ok(Some(quote! {
        format!("{}{}: {},", " ".repeat((indent + 1) * 4), #name, #ty)
    }))
}
