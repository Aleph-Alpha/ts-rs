use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Generics, ItemEnum, Variant};

use crate::{
    attr::{EnumAttr, FieldAttr, StructAttr, Tagged, VariantAttr},
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

    let mut formatted_variants = Vec::new();
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

    let flattened_structs = s.variants.iter().flat_map(|x| {
        let variant_attr = VariantAttr::from_attrs(&x.attrs).ok()?;

        if variant_attr.skip {
            return None;
        }

        let variant_type = types::type_def(
            &StructAttr::from(variant_attr),
            &x.ident,
            &x.fields,
            &s.generics,
        ).ok()?;

        if variant_type.inline_flattened.is_none() {
            None
        } else {
            Some(variant_type)
        }
    }).collect::<Box<[_]>>();

    let generic_args = format_generics(&mut dependencies, &s.generics);
    let inline_flattened = match enum_attr.tagged()? {
        Tagged::Externally => {
            let flattened_structs = flattened_structs
                .iter()
                .map(|x| {
                    let ident = &x.name;
                    let flattened = &x.inline_flattened;
                    quote!(format!(r#""{}": {{ {} }},"#, #ident, #flattened))
                });
            Some(quote!(<[String]>::join(&[#(#flattened_structs),*], " } | { ")))
        },
        Tagged::Adjacently { tag, content } => {
            let flattened_structs = flattened_structs
                .iter()
                .map(|x| {
                    let ident = &x.name;
                    let flattened = &x.inline_flattened;
                    quote!(format!(r#""{}": "{}", "{}": {{ {} }},"#, #tag, #ident, #content, #flattened))
                });
            Some(quote!(<[String]>::join(&[#(#flattened_structs),*], " } | { ")))
        },
        Tagged::Internally { tag } => {
            if flattened_structs.is_empty() {
                None
            } else {
                let flattened_structs = flattened_structs
                    .iter()
                    .map(|x| {
                        let ident = &x.name;
                        let flattened = &x.inline_flattened;
                        quote!(format!(r#""{}": "{}", {}"#, #tag, #ident, #flattened))
                    });
                Some(quote!(<[String]>::join(&[#(#flattened_structs),*], " } | { ")))
            }
        },
        Tagged::Untagged => {
            if flattened_structs.is_empty() {
                None
            } else {
                let flattened_structs = flattened_structs
                    .iter()
                    .map(|x| x.inline_flattened.clone());
                Some(quote!(<[String]>::join(&[#(#flattened_structs),*], " } | { ")))
            }
        },
    };

    Ok(DerivedTS {
        inline: quote!([#(#formatted_variants),*].join(" | ")),
        decl: quote!(format!("type {}{} = {};", #name, #generic_args, Self::inline())),
        inline_flattened,
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
    let variant_attr = VariantAttr::from_attrs(&variant.attrs)?;

    if variant_attr.skip {
        return Ok(());
    }

    let name = match (variant_attr.rename.clone(), &enum_attr.rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    let variant_type = types::type_def(
        &StructAttr::from(variant_attr),
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
        Tagged::Adjacently { tag, content } => match &variant.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let FieldAttr {
                    type_override,
                    skip,
                    ..
                } = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;
                if skip {
                    quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name))
                } else {
                    let ty = if let Some(type_override) = type_override {
                        quote! { #type_override }
                    } else {
                        format_type(&unnamed.unnamed[0].ty, dependencies, generics)
                    };
                    quote!(format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #name, #content, #ty))
                }
            }
            Fields::Unit => quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name)),
            _ => quote!(
                format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #name, #content, #inline_type)
            ),
        },
        Tagged::Internally { tag } => match variant_type.inline_flattened {
            Some(inline_flattened) => quote! {
                format!(
                    "{{ \"{}\": \"{}\", {} }}",
                    #tag,
                    #name,
                    #inline_flattened
                )
            },
            None => match &variant.fields {
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let FieldAttr {
                        type_override,
                        skip,
                        ..
                    } = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;
                    if skip {
                        quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name))
                    } else {
                        let ty = if let Some(type_override) = type_override {
                            quote! { #type_override }
                        } else {
                            format_type(&unnamed.unnamed[0].ty, dependencies, generics)
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
