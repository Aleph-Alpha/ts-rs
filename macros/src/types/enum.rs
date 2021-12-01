use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Generics, ItemEnum, Variant};

use crate::{
    attr::{EnumAttr, FieldAttr, StructAttr, Tagged},
    deps::Dependencies,
    types,
    types::generics::{format_generics, format_type},
    DerivedTS,
};

pub(crate) fn r#enum_def(s: &ItemEnum) -> syn::Result<DerivedTS> {
    let enum_attr: EnumAttr = EnumAttr::from_attrs(&s.attrs)?;

    let name = match &enum_attr.rename {
        Some(existing) => existing.clone(),
        None => s.ident.to_string(),
    };

    if s.variants.is_empty() {
        return Ok(empty_enum(name, enum_attr));
    }

    if s.variants.is_empty() {
        return Ok(DerivedTS {
            name,
            inline: quote!("never".to_owned()),
            decl: quote!("type {} = never;"),
            inline_flattened: None,
            dependencies: Dependencies::default(),
            export: enum_attr.export,
            export_to: enum_attr.export_to,
        });
    }

    let mut formatted_variants = vec![];
    let mut dependencies = Dependencies::default();
    for variant in &s.variants {
        format_variant(
            &mut formatted_variants,
            &mut dependencies,
            &enum_attr,
            variant,
            &s.generics,
        )?;
    }

    let generic_args = format_generics(&mut dependencies, &s.generics);
    Ok(DerivedTS {
        inline: quote!(vec![#(#formatted_variants),*].join(" | ")),
        decl: quote!(format!("type {}{} = {};", #name, #generic_args, Self::inline())),
        inline_flattened: None,
        dependencies,
        name,
        export: enum_attr.export,
        export_to: enum_attr.export_to,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    enum_attr: &EnumAttr,
    variant: &Variant,
    generics: &Generics,
) -> syn::Result<()> {
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
        (_, Some(_), ..) => syn_err!("`type` is not applicable to enum variants"),
        (_, _, _, true, ..) => syn_err!("`optional` is not applicable to enum variants"),
        (_, _, _, _, true) => syn_err!("`flatten` is not applicable to enum variants"),
        _ => {}
    };

    let name = match (rename, &enum_attr.rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    let variant_type = types::type_def(
        &StructAttr::default(),
        // since we are generating the variant as a struct, it doesn't have a name
        &format_ident!("_"),
        &variant.fields,
        generics,
    )?;
    let variant_dependencies = variant_type.dependencies;
    let inline_type = variant_type.inline;

    let formatted = match enum_attr.tagged()? {
        Tagged::Untagged => quote!(#inline_type),
        Tagged::Externally => match &variant.fields {
            Fields::Unit => quote!(format!("\"{}\"", #name)),
            _ => quote!(format!("{{ {}: {} }}", #name, #inline_type)),
        },
        Tagged::Adjacently { tag, content } => match &variant.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let ty = format_type(&unnamed.unnamed[0].ty, dependencies, generics);
                quote!(format!("{{ {}: \"{}\", {}: {} }}", #tag, #name, #content, #ty))
            }
            Fields::Unit => quote!(format!("{{ {}: \"{}\" }}", #tag, #name)),
            _ => quote!(format!("{{ {}: \"{}\", {}: {} }}", #tag, #name, #content, #inline_type)),
        },
        Tagged::Internally { tag } => match variant_type.inline_flattened {
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
                    let ty = format_type(&unnamed.unnamed[0].ty, dependencies, generics);
                    quote!(format!("{{ {}: \"{}\" }} & {}", #tag, #name, #ty))
                }
                Fields::Unit => quote!(format!("{{ {}: \"{}\" }}", #tag, #name)),
                _ => {
                    quote!(format!("{{ {}: \"{}\" }} & {}", #tag, #name, #inline_type))
                }
            },
        },
    };

    dependencies.append(variant_dependencies);
    formatted_variants.push(formatted);
    Ok(())
}

// bindings for an empty enum (`never` in TS)
fn empty_enum(name: impl Into<String>, enum_attr: EnumAttr) -> DerivedTS {
    let name = name.into();
    DerivedTS {
        inline: quote!("never".to_owned()),
        decl: quote!(format!("type {} = never;", #name)),
        name,
        inline_flattened: None,
        dependencies: Dependencies::default(),
        export: enum_attr.export,
        export_to: enum_attr.export_to,
    }
}
