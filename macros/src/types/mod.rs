use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Generics, ItemEnum, ItemStruct, Result, Variant};

use crate::{
    attr::{EnumAttr, FieldAttr, Inflection, StructAttr},
    deps::Dependencies,
    types::generics::format_type,
    utils::to_ts_ident,
    DerivedTS,
};

mod generics;
mod named;
mod newtype;
mod tuple;
mod unit;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let StructAttr { rename_all, rename } = StructAttr::from_attrs(&s.attrs)?;
    let name = rename.unwrap_or_else(|| to_ts_ident(&s.ident));

    type_def(&name, &rename_all, &s.fields, &s.generics)
}

fn type_def(
    name: &str,
    rename_all: &Option<Inflection>,
    fields: &Fields,
    generics: &Generics,
) -> Result<DerivedTS> {
    match fields {
        Fields::Named(named) => match named.named.len() {
            0 => unit::unit(name, rename_all),
            _ => named::named(name, rename_all, named, generics),
        },
        Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => unit::unit(name, rename_all),
            1 => newtype::newtype(name, rename_all, unnamed, generics),
            _ => tuple::tuple(name, rename_all, unnamed, generics),
        },
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
    let mut dependencies = Dependencies::default();
    for variant in &s.variants {
        format_variant(
            &mut formatted_variants,
            &mut dependencies,
            &enum_attr,
            variant,
        )?;
    }

    Ok(DerivedTS {
        inline: quote!(vec![#(#formatted_variants),*].join(" | ")),
        decl: quote!(format!("type {} = {};", #name, Self::inline())),
        inline_flattened: None,
        dependencies,
        name,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    enum_attr: &EnumAttr,
    variant: &Variant,
) -> Result<()> {
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip,
        optional,
        flatten,
    } = FieldAttr::from_attrs(&variant.attrs)?;

    match (skip, &type_override, inline, optional, flatten) {
        (true, ..) => return Ok(()),
        (_, Some(_), ..) => syn_err!("`type_override` is not applicable to enum variants"),
        (_, _, _, true, ..) => syn_err!("`optional` is not applicable to enum variants"),
        (_, _, _, _, true) => syn_err!("`flatten` is not applicable to enum variants"),
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

    let generics = Generics::default();
    let variant_type = type_def(&name, &None, &variant.fields, &generics)?;
    let variant_dependencies = variant_type.dependencies;
    let inline_type = variant_type.inline;

    formatted_variants.push(match &enum_attr.untag {
        true => quote!(#inline_type),
        false => match &enum_attr.tag {
            Some(tag) => match &enum_attr.content {
                Some(content) => match &variant.fields {
                    Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                        let ty = format_type(&unnamed.unnamed[0].ty, dependencies, &generics);
                        quote!(format!("{{ {}: \"{}\", {}: {} }}", #tag, #name, #content, #ty))
                    }
                    Fields::Unit => quote!(format!("{{ {}: \"{}\" }}", #tag, #name)),
                    _ => quote!(
                        format!("{{ {}: \"{}\", {}: {} }}", #tag, #name, #content, #inline_type)
                    ),
                },
                None => match variant_type.inline_flattened {
                    Some(inline_flattened) => quote! {
                        format!(
                            "{{ {}: \"{}\", {} }}",
                            #tag,
                            #name,
                            #inline_flattened
                        )
                    },
                    None => match &variant.fields {
                        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                            let ty = format_type(&unnamed.unnamed[0].ty, dependencies, &generics);
                            quote!(format!("{{ {}: \"{}\" }} & {}", #tag, #name, #ty))
                        }
                        Fields::Unit => quote!(format!("{{ {}: \"{}\" }}", #tag, #name)),
                        _ => {
                            dependencies.append(variant_dependencies);
                            quote!(format!("{{ {}: \"{}\" }} & {}", #tag, #name, #inline_type))
                        }
                    },
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
