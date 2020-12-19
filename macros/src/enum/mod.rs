use quote::quote;
use syn::{spanned::Spanned, Fields, ItemEnum, Result, Variant};

use crate::attr::{EnumAttr, FieldAttr, Inflection};
use crate::DerivedTS;

pub(crate) fn r#enum(s: &ItemEnum) -> Result<DerivedTS> {
    let EnumAttr { rename_all, rename } = EnumAttr::from_attrs(&s.attrs)?;

    if let Some(v) = s
        .variants
        .iter()
        .find(|v| !matches!(v.fields, Fields::Unit))
    {
        syn_err!(v.span(); "variant has data attached. Such enums are not yet supported.");
    }

    let name = rename.unwrap_or_else(|| s.ident.to_string());
    let mut formatted_variants = vec![];
    for variant in &s.variants {
        format_variant(&mut formatted_variants, &rename_all, &variant)?;
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
    formatted_variants: &mut Vec<String>,
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
        (_, _, true, ..) => syn_err!("`inline` is not applicable to enum variants"),
        (_, _, _, true) => syn_err!("`flatten` is not applicable to enum variants"),
        _ => {}
    };

    let name = match (rename, &rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    formatted_variants.push(format!("{:?}", name));
    Ok(())
}
