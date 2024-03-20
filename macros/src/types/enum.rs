use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemEnum, Variant};

use crate::{
    attr::{EnumAttr, FieldAttr, StructAttr, Tagged, VariantAttr},
    deps::Dependencies,
    types, DerivedTS,
};

pub(crate) fn r#enum_def(s: &ItemEnum) -> syn::Result<DerivedTS> {
    let enum_attr: EnumAttr = EnumAttr::from_attrs(&s.attrs)?;

    let crate_rename = enum_attr.crate_rename();

    let name = match &enum_attr.rename {
        Some(existing) => existing.clone(),
        None => s.ident.to_string(),
    };

    if s.variants.is_empty() {
        return Ok(empty_enum(name, enum_attr));
    }

    if s.variants.is_empty() {
        return Ok(DerivedTS {
            crate_rename: crate_rename.clone(),
            ts_name: name,
            docs: enum_attr.docs,
            inline: quote!("never".to_owned()),
            inline_flattened: None,
            dependencies: Dependencies::new(crate_rename),
            export: enum_attr.export,
            export_to: enum_attr.export_to,
            concrete: enum_attr.concrete,
            bound: enum_attr.bound,
        });
    }

    let mut formatted_variants = Vec::new();
    let mut dependencies = Dependencies::new(crate_rename.clone());
    for variant in &s.variants {
        format_variant(
            &mut formatted_variants,
            &mut dependencies,
            &enum_attr,
            variant,
        )?;
    }

    Ok(DerivedTS {
        crate_rename,
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
        bound: enum_attr.bound,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    enum_attr: &EnumAttr,
    variant: &Variant,
) -> syn::Result<()> {
    let crate_rename = enum_attr.crate_rename();
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

    let struct_attr = StructAttr::from_variant(enum_attr, &variant_attr);
    let variant_type = types::type_def(
        &struct_attr,
        // since we are generating the variant as a struct, it doesn't have a name
        &format_ident!("_"),
        &variant.fields,
    )?;
    let variant_dependencies = variant_type.dependencies;
    let inline_type = variant_type.inline;

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
                        (Some(_), Some(_)) => {
                            syn_err_spanned!(variant; "`type` is not compatible with `as`")
                        }
                        (Some(type_override), None) => quote! { #type_override },
                        (None, Some(type_as)) => {
                            quote!(<#type_as as #crate_rename::TS>::name())
                        }
                        (None, None) => {
                            let ty = &unnamed.unnamed[0].ty;
                            quote!(<#ty as #crate_rename::TS>::name())
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
                            (Some(_), Some(_)) => {
                                syn_err_spanned!(variant; "`type` is not compatible with `as`")
                            }
                            (Some(type_override), None) => quote! { #type_override },
                            (None, Some(type_as)) => {
                                quote!(<#type_as as #crate_rename::TS>::name())
                            }
                            (None, None) => {
                                let ty = &unnamed.unnamed[0].ty;
                                quote!(<#ty as #crate_rename::TS>::name())
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
fn empty_enum(name: impl Into<String>, enum_attr: EnumAttr) -> DerivedTS {
    let name = name.into();
    let crate_rename = enum_attr.crate_rename();
    DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("never".to_owned()),
        docs: enum_attr.docs,
        inline_flattened: None,
        dependencies: Dependencies::new(crate_rename),
        export: enum_attr.export,
        export_to: enum_attr.export_to,
        ts_name: name,
        concrete: enum_attr.concrete,
        bound: enum_attr.bound,
    }
}
