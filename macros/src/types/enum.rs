use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Generics, ItemEnum, Variant, Ident};

use crate::{
    attr::{EnumAttr, FieldAttr, StructAttr, Tagged, VariantAttr},
    deps::Dependencies,
    types, DerivedTS,
};

pub(crate) fn r#enum_def(s: &ItemEnum) -> syn::Result<DerivedTS> {
    let enum_attr: EnumAttr = EnumAttr::from_attrs(&s.attrs)?;

    let name = match &enum_attr.rename {
        Some(existing) => existing.clone(),
        None => s.ident.to_string(),
    };

    if s.variants.is_empty() {
        return Ok(empty_enum(name, enum_attr, s.generics.clone()));
    }

    if s.variants.is_empty() {
        return Ok(DerivedTS {
            generics: s.generics.clone(),
            ts_name: name,
            docs: enum_attr.docs,
            inline: quote!("never".to_owned()),
            inline_flattened: None,
            dependencies: Dependencies::default(),
            export: enum_attr.export,
            export_to: enum_attr.export_to,
            concrete: enum_attr.concrete,
            extra_ts_bounds: HashSet::default(),
        });
    }

    let mut extra_ts_bounds = HashSet::new();
    let mut formatted_variants = Vec::new();
    let mut dependencies = Dependencies::default();
    for variant in &s.variants {
        format_variant(
            &mut formatted_variants,
            &mut dependencies,
            &mut extra_ts_bounds,
            &enum_attr,
            variant,
            &s.generics,
        )?;
    }

    Ok(DerivedTS {
        generics: s.generics.clone(),
        inline: quote!([#(#formatted_variants),*].join(" | ")),
        inline_flattened: Some(quote!(
            format!("({})", [#(#formatted_variants),*].join(" | "))
        )),
        dependencies,
        docs: enum_attr.docs,
        export: enum_attr.export,
        export_to: enum_attr.export_to,
        ts_name: name,
        concrete: enum_attr.concrete,
        extra_ts_bounds,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    extra_ts_bounds: &mut HashSet<Ident>,
    enum_attr: &EnumAttr,
    variant: &Variant,
    generics: &Generics,
) -> syn::Result<()> {
    let variant_attr = VariantAttr::new(&variant.attrs, enum_attr)?;

    if variant_attr.skip {
        return Ok(());
    }

    let untagged_variant = variant_attr.untagged;
    let name = match (variant_attr.rename.clone(), &enum_attr.rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    let mut attr = StructAttr::from(variant_attr);
    attr.concrete = enum_attr.concrete.clone();
    let variant_type = types::type_def(
        &attr,
        // since we are generating the variant as a struct, it doesn't have a name
        &format_ident!("_"),
        &variant.fields,
        generics,
    )?;
    let variant_dependencies = variant_type.dependencies;
    let inline_type = variant_type.inline;

    extra_ts_bounds.extend(variant_type.extra_ts_bounds);

    let formatted = match (untagged_variant, enum_attr.tagged()?) {
        (true, _) | (_, Tagged::Untagged) => quote!(#inline_type),
        (false, Tagged::Externally) => match &variant.fields {
            Fields::Unit => quote!(format!("\"{}\"", #name)),
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let FieldAttr { skip, .. } = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;
                if skip {
                    quote!(format!("\"{}\"", #name))
                } else {
                    quote!(format!("{{ \"{}\": {} }}", #name, #inline_type))
                }
            }
            _ => quote!(format!("{{ \"{}\": {} }}", #name, #inline_type)),
        },
        (false, Tagged::Adjacently { tag, content }) => match &variant.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let FieldAttr {
                    type_as,
                    type_override,
                    skip,
                    ..
                } = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;

                if skip {
                    quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name))
                } else {
                    let ty = match (type_override, type_as) {
                        (Some(_), Some(_)) => syn_err_spanned!(variant; "`type` is not compatible with `as`"),
                        (Some(type_override), None) => quote! { #type_override },
                        (None, Some(type_as)) => {
                            quote!(<#type_as as ts_rs::TS>::name())
                        }
                        (None, None) => {
                            let ty = &unnamed.unnamed[0].ty;
                            quote!(<#ty as ts_rs::TS>::name())
                        }
                    };

                    quote!(format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #name, #content, #ty))
                }
            }
            Fields::Unit => quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name)),
            _ => quote!(
                format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #name, #content, #inline_type)
            ),
        },
        (false, Tagged::Internally { tag }) => match variant_type.inline_flattened {
            Some(inline_flattened) => quote! {
                format!(
                    "{{ \"{}\": \"{}\", {} }}",
                    #tag,
                    #name,
                    // At this point inline_flattened looks like
                    // { /* ...data */ }
                    //
                    // To be flattened, an internally tagged enum must not be
                    // surrounded by braces, otherwise each variant will look like
                    // { "tag": "name", { /* ...data */ } }
                    // when we want it to look like
                    // { "tag": "name", /* ...data */ }
                    #inline_flattened.trim_matches(&['{', '}', ' '])
                )
            },
            None => match &variant.fields {
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let FieldAttr {
                        type_as,
                        skip,
                        type_override,
                        ..
                    } = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;

                    if skip {
                        quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name))
                    } else {
                        let ty = match (type_override, type_as) {
                            (Some(_), Some(_)) => syn_err_spanned!(variant; "`type` is not compatible with `as`"),
                            (Some(type_override), None) => quote! { #type_override },
                            (None, Some(type_as)) => {
                                quote!(<#type_as as ts_rs::TS>::name())
                            }
                            (None, None) => {
                                let ty = &unnamed.unnamed[0].ty;
                                quote!(<#ty as ts_rs::TS>::name())
                            }
                        };

                        quote!(format!("{{ \"{}\": \"{}\" }} & {}", #tag, #name, #ty))
                    }
                }
                Fields::Unit => quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name)),
                _ => {
                    quote!(format!("{{ \"{}\": \"{}\" }} & {}", #tag, #name, #inline_type))
                }
            },
        },
    };

    dependencies.append(variant_dependencies);
    formatted_variants.push(formatted);
    Ok(())
}

// bindings for an empty enum (`never` in TS)
fn empty_enum(name: impl Into<String>, enum_attr: EnumAttr, generics: Generics) -> DerivedTS {
    let name = name.into();
    DerivedTS {
        generics,
        inline: quote!("never".to_owned()),
        docs: enum_attr.docs,
        inline_flattened: None,
        dependencies: Dependencies::default(),
        export: enum_attr.export,
        export_to: enum_attr.export_to,
        ts_name: name,
        concrete: enum_attr.concrete,
        extra_ts_bounds: HashSet::default(),
    }
}
