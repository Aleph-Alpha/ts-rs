use quote::quote;
use syn::{Fields, ItemEnum, ItemStruct, Result, Variant};
use proc_macro2::{TokenStream};

use crate::attr::{EnumAttr, FieldAttr, Inflection, StructAttr};
use crate::DerivedTS;

mod named;
mod newtype;
mod tuple;
mod unit;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let StructAttr {
        rename_all,
        rename,
    } = StructAttr::from_attrs(&s.attrs)?;
    let name = rename.unwrap_or_else(|| s.ident.to_string());

    type_def(&name, &rename_all, &s.fields)
}

fn type_def(name: &String, rename_all: &Option<Inflection>, fields: &Fields) -> Result<DerivedTS> {
    match fields {
        Fields::Named(named) => named::named(name, rename_all, &named),
        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => newtype::newtype(name, rename_all, &unnamed),
        Fields::Unnamed(unnamed) => tuple::tuple(name, rename_all, &unnamed),
        Fields::Unit => unit::unit(name, rename_all),
    }
}


pub(crate) fn r#enum(s: &ItemEnum) -> Result<DerivedTS> {
    let EnumAttr { rename_all, rename, tag } = EnumAttr::from_attrs(&s.attrs)?;

    let name = rename.unwrap_or_else(|| s.ident.to_string());
    let mut formatted_variants = vec![];
    for variant in &s.variants {
        format_variant(&mut formatted_variants, &tag, &rename_all, &variant)?;
    }

    Ok(DerivedTS {
        inline: quote!(vec![#(#formatted_variants),*].join(" | ")),
        decl: quote!(format!("export type {} = {};", #name, Self::inline(0))),
        inline_flattened: None,
        dependencies: quote!((vec![])),
        name,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    tag: &Option<String>,
    rename_all: &Option<Inflection>,
    variant: &Variant,
) -> Result<()> {
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip,
        flatten,
    } = FieldAttr::from_attrs(&variant.attrs)?;
    
    match (skip, &type_override, inline, flatten) {
        (true, ..) => return Ok(()),
        (_, Some(_), ..) => syn_err!("`type_override` is not applicable to enum variants"),
        (_, _, _, true) => syn_err!("`flatten` is not applicable to enum variants"),
        _ => {}
    };

    let name = match (rename, &rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    formatted_variants.push( match tag {
        Some(tag_name) => {
            let structd = format!("{{{}: {:?}}}", tag_name, name);
            quote!(#structd)
        },
        None => match &variant.fields {
            Fields::Unit => {
                let quoted_name = format!("\"{}\"", name);
                quote!(#quoted_name.to_owned())
            },
            Fields::Unnamed(unnamed) => {
                let ty = &unnamed.unnamed;
                quote!{<#ty as ts_rs::TS>::inline(0)}
            },
            _ => panic!("Don't support variants with these kind of fields")
        }
    });
    Ok(())
}
