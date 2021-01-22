use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum, ItemStruct, Result, Variant};

use crate::attr::{EnumAttr, FieldAttr, Inflection, StructAttr};
use crate::DerivedTS;

mod named;
mod newtype;
mod tuple;
mod unit;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let StructAttr { rename_all, rename } = StructAttr::from_attrs(&s.attrs)?;
    let name = rename.unwrap_or_else(|| s.ident.to_string());

    type_def(&name, &rename_all, &s.fields)
}

fn type_def(name: &String, rename_all: &Option<Inflection>, fields: &Fields) -> Result<DerivedTS> {
    match fields {
        Fields::Named(named) => named::named(name, rename_all, &named),
        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
            newtype::newtype(name, rename_all, &unnamed)
        }
        Fields::Unnamed(unnamed) => tuple::tuple(name, rename_all, &unnamed),
        Fields::Unit => unit::unit(name, rename_all),
    }
}

pub(crate) fn r#enum(s: &ItemEnum) -> Result<DerivedTS> {
    let enum_attr: EnumAttr = EnumAttr::from_attrs(&s.attrs)?;

    let name = match &enum_attr.rename {
        Some(existing) => existing.clone(),
        None => s.ident.to_string(),
    };

    let mut formatted_variants = vec![];
    for variant in &s.variants {
        format_variant(&mut formatted_variants, &enum_attr, &variant)?;
    }

    Ok(DerivedTS {
        inline: quote!(vec![#(#formatted_variants),*].join(" |\n")),
        decl: quote!(format!("export type {} = {};", #name, Self::inline(0))),
        inline_flattened: None,
        dependencies: quote!((vec![])),
        name,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    enum_attr: &EnumAttr,
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

    let name = match (rename, &enum_attr.rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    match (&enum_attr.tag, &enum_attr.content, &enum_attr.untag) {
        (_, Some(_), true) => panic!("Untagged enums cannot have content tags"),
        (Some(_), _, true) => panic!("Untagged enums cannot have tags"),
        _ => {}
    };

    let inline_type = type_def(&name, &None, &variant.fields)?.inline;

    formatted_variants.push(match &enum_attr.untag {
        true => quote!(#inline_type),
        false => match &enum_attr.tag {
            Some(tag) => match &enum_attr.content {
                Some(content) => {
                    quote!(format!("{{{}: \"{}\", {}: {}}}", #tag, #name, #content, #inline_type))
                }
                None => panic!("Serde enums with tag discriminators should also have content keys"),
            },
            None => match &variant.fields {
                Fields::Unit => quote!(format!("\"{}\"", #name)),
                _ => quote!(#inline_type),
            },
        },
    });
    Ok(())
}
