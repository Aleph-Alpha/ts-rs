use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsNamed, ItemStruct, Result};

use crate::attr::FieldAttr;
use crate::DerivedTS;

pub(crate) fn named(s: &ItemStruct, i: &FieldsNamed) -> Result<DerivedTS> {
    let name = s.ident.to_string();
    let fields = i
        .named
        .iter()
        .map(format_field)
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

fn format_field(field: &Field) -> Result<TokenStream> {
    let FieldAttr {
        type_override,
        rename,
        inline,
    } = FieldAttr::from_attrs(&field.attrs)?;

    let ty = &field.ty;
    let ty = type_override
        .map(|t| quote!(#t))
        .unwrap_or_else(|| quote!(<#ty as ts_rs::TS>::format(indent + 1, #inline)));
    let name = rename.unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());

    Ok(quote! {
        format!("{}{}: {},", " ".repeat((indent + 1) * 4), #name, #ty)
    })
}
