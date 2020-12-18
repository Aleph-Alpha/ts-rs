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
        .flat_map(Result::transpose)
        .collect::<Result<Vec<TokenStream>>>()?;
    let formatted_fields = quote!(vec![#(#fields),*].join("\n"));

    Ok(DerivedTS {
        inline: quote! {
            format!(
                "{{\n{}\n{}}}",
                #formatted_fields,
                " ".repeat(indent * 4)
            )
        },
        decl: quote!(format!("export interface {} {}", #name, Self::inline(0))),
        inline_flattened: Some(formatted_fields),
        name,
    })
}

// build an expresion which expands to a string, representing a single field of a struct.
fn format_field(field: &Field, rename_all: &Option<Inflection>) -> Result<Option<TokenStream>> {
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

    let ty = &field.ty;

    if flatten {
        match (&type_override, &rename, inline) {
            (Some(_), _, _) => syn_err!("`type` is not compatible with `flatten`"),
            (_, Some(_), _) => syn_err!("`rename` is not compatible with `flatten`"),
            (_, _, true) => syn_err!("`inline` is not compatible with `flatten`"),
            _ => {}
        }
        return Ok(Some(quote!(<#ty as ts_rs::TS>::inline_flattened(indent))));
    }

    let ty = type_override
        .map(|t| quote!(#t))
        .unwrap_or_else(|| {
            match inline {
                true => quote!(<#ty as ts_rs::TS>::inline(indent + 1)),
                false => quote!(<#ty as ts_rs::TS>::name()),
            }
        });
    let name = match (rename, rename_all) {
        (Some(rn), _) => rn,
        (None, Some(rn)) => rn.apply(&field.ident.as_ref().unwrap().to_string()),
        (None, None) => field.ident.as_ref().unwrap().to_string(),
    };

    Ok(Some(quote! {
        format!("{}{}: {},", " ".repeat((indent + 1) * 4), #name, #ty)
    }))
}
