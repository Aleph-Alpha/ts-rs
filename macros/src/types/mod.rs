use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
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

fn type_def(name: &str, rename_all: &Option<Inflection>, fields: &Fields) -> Result<DerivedTS> {
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

    let is_enum = match enum_attr.r#type.as_deref() {
        Some("enum" | "const enum") => { true },
        None | Some("type") => { false },
        Some(x) => {
            syn_err!("Either `const enum`, `enum` or `type` accepted; was: {:?}", x);
        }
    };

    let mut formatted_variants = vec![];
    if is_enum {
        let any_renamed = enum_attr.rename_all.is_some() || s.variants.iter().find(|v| {
            let FieldAttr {
                rename,
                ..
            } = FieldAttr::from_attrs(&v.attrs).unwrap();
            rename.is_some()
        }).is_some();

        for variant in &s.variants {
            format_enum_variant(&mut formatted_variants, &enum_attr, &variant, any_renamed)?;
        }
    } else {
        for variant in &s.variants {
            format_variant(&mut formatted_variants, &enum_attr, &variant)?;
        }
    }

    let inline = if is_enum {
        quote!([#(#formatted_variants),*].join(", "))
    } else {
        quote!([#(#formatted_variants),*].join(" | "))
    };

    let overwrite_type = enum_attr.r#type.unwrap_or(String::from("type"));

    let decl = if is_enum {
        quote!(format!("{} {} {{{}}}", #overwrite_type, #name, Self::inline(0)))
    } else {
        quote!(format!("{} {} = {};", #overwrite_type, #name, Self::inline(0)))
    };

    Ok(DerivedTS {
        inline,
        decl,
        inline_flattened: None,
        dependencies: quote!((vec![])),
        name,
    })
}

/// If any have been renamed then we want to rename all enum variants
fn format_enum_variant(
    formatted_variants: &mut Vec<TokenStream>,
    enum_attr: &EnumAttr,
    variant: &Variant,
    any_renamed: bool
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
        (_, _, true, _) => syn_err!("`inline` is not applicable to enum variants when type enum"),
        _ => {}
    };

    let name = variant.ident.to_string();

    let enum_renamed_value: Option<String> = match (rename, &enum_attr.rename_all, any_renamed) {
        (Some(rn), _, _) => Some(rn),
        (None, None, true) => Some(name.clone()),
        (None, None, false) => None,
        (None, Some(rn), _) => Some(rn.apply(&name)),
    };

    for (forbidden_attr_name, forbidden_attr_val) in [
        ("tag", &enum_attr.tag.as_deref()),
        ("content", &enum_attr.content.as_deref()),
        ("untag", &enum_attr.untag.then(|| "true"))
    ] {
        if let Some(_) = forbidden_attr_val {
            syn_err!("Invalid enum attribute {:?} when type is enum.", forbidden_attr_name)
        }
    }

    formatted_variants.push(if let Some((_, expr)) = &variant.discriminant {
        let str = format!("{}={}", name, expr.to_token_stream());
        quote!(#str)
    } else if let Some(renamed) = enum_renamed_value {
        if let Some((_, e)) = &variant.discriminant {
            if !any_renamed {
                syn_err!("{:?} Can't be both renamed and have a discriminant {:?}", name, e.to_token_stream());
            }
        }
        let str =  format!("{}=\"{}\"", name, renamed);
        quote!(#str)
    } else {
        quote!(#name)
    });
    Ok(())
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
        (_, Some(_), true) => syn_err!("Untagged enums cannot have content tags"),
        (Some(_), _, true) => syn_err!("Untagged enums cannot have tags"),
        _ => {}
    };

    let derived_type = type_def(&name, &None, &variant.fields)?;
    let inline_type = derived_type.inline;

    formatted_variants.push(match &enum_attr.untag {
        true => quote!(#inline_type),
        false => match &enum_attr.tag {
            Some(tag) => match &enum_attr.content {
                Some(content) => {
                    quote!(format!("{{{}: \"{}\", {}: {}}}", #tag, #name, #content, #inline_type))
                }
                None => match derived_type.inline_flattened {
                    Some(inline_flattened) => quote! {
                        format!(
                            "{{\n{}{}: \"{}\",\n{}\n}}",
                            " ".repeat((indent + 1) * 4),
                            #tag,
                            #name,
                            #inline_flattened
                        )
                    },
                    None => syn_err!(
                        "Serde enums with tag discriminators should also have flattened fields"
                    ),
                },
            },
            None => match &variant.fields {
                Fields::Unit => quote!(format!("\"{}\"", #name)),
                _ => quote!(#inline_type),
            },
        },
    });
    Ok(())
}
