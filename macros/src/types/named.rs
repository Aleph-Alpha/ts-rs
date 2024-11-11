use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Field, FieldsNamed, Path, Result};

use crate::{
    attr::{Attr, ContainerAttr, FieldAttr, Inflection, Optional, StructAttr},
    deps::Dependencies,
    utils::{raw_name_to_ts_field, to_ts_ident},
    DerivedTS,
};

pub(crate) fn named(attr: &StructAttr, name: &str, fields: &FieldsNamed) -> Result<DerivedTS> {
    let crate_rename = attr.crate_rename();

    let mut formatted_fields = Vec::new();
    let mut flattened_fields = Vec::new();
    let mut dependencies = Dependencies::new(crate_rename.clone());

    if let Some(tag) = &attr.tag {
        let formatted = format!("\"{}\": \"{}\",", tag, name);
        formatted_fields.push(quote! {
            #formatted.to_string()
        });
    }

    for field in &fields.named {
        format_field(
            &crate_rename,
            &mut formatted_fields,
            &mut flattened_fields,
            &mut dependencies,
            field,
            &attr.rename_all,
            attr.optional,
        )?;
    }

    let fields = quote!(<[String]>::join(&[#(#formatted_fields),*], " "));
    let flattened = quote!(<[String]>::join(&[#(#flattened_fields),*], " & "));

    let inline = match (formatted_fields.len(), flattened_fields.len()) {
        (0, 0) => quote!("{  }".to_owned()),
        (_, 0) => quote!(format!("{{ {} }}", #fields)),
        (0, 1) => quote! {{
            if #flattened.starts_with('(') && #flattened.ends_with(')') {
                #flattened[1..#flattened.len() - 1].trim().to_owned()
            } else {
                #flattened.trim().to_owned()
            }
        }},
        (0, _) => quote!(#flattened),
        (_, _) => quote!(format!("{{ {} }} & {}", #fields, #flattened)),
    };

    let inline_flattened = match (formatted_fields.len(), flattened_fields.len()) {
        (0, 0) => quote!("{  }".to_owned()),
        (_, 0) => quote!(format!("{{ {} }}", #fields)),
        (0, _) => quote!(#flattened),
        (_, _) => quote!(format!("{{ {} }} & {}", #fields, #flattened)),
    };

    Ok(DerivedTS {
        crate_rename,
        // the `replace` combines `{ ... } & { ... }` into just one `{ ... }`. Not necessary, but it
        // results in simpler type definitions.
        inline: quote!(#inline.replace(" } & { ", " ")),
        inline_flattened: Some(quote!(#inline_flattened.replace(" } & { ", " "))),
        docs: attr.docs.clone(),
        dependencies,
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

// build an expression which expands to a string, representing a single field of a struct.
//
// formatted_fields will contain all the fields that do not contain the flatten
// attribute, in the format
// key: type,
//
// flattened_fields will contain all the fields that contain the flatten attribute
// in their respective formats, which for a named struct is the same as formatted_fields,
// but for enums is
// ({ /* variant data */ } | { /* variant data */ })
fn format_field(
    crate_rename: &Path,
    formatted_fields: &mut Vec<TokenStream>,
    flattened_fields: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    field: &Field,
    rename_all: &Option<Inflection>,
    struct_optional: Optional,
) -> Result<()> {
    let field_attr = FieldAttr::from_attrs(&field.attrs)?;

    field_attr.assert_validity(field)?;

    if field_attr.skip {
        return Ok(());
    }

    let ty = field_attr.type_as(&field.ty);

    let (optional_annotation, nullable) = match (struct_optional, field_attr.optional) {
        // `#[ts(optional)]` on field takes precedence, and is enforced **AT COMPILE TIME**
        (_, Optional::Optional { nullable }) => (
            // expression that evaluates to the string "?", but fails to compile if `ty` is not an `Option`.
            quote! {{
                use std::marker::PhantomData;
                let actual: PhantomData<#ty> = PhantomData;
                let must: PhantomData<std::option::Option<_>> = actual;
                "?"
            }},
            nullable,
        ),
        // `#[ts(optional)]` on the struct acts as `#[ts(optional)]` on a field, but does not error on non-`Option`
        // fields. Instead, it is a no-op.
        (Optional::Optional { nullable }, _) => (
            quote! {
                if <#ty as #crate_rename::TS>::IS_OPTION { "?" } else { "" }
            },
            nullable,
        ),
        _ => (quote!(""), true),
    };

    let ty = if nullable {
        ty
    } else {
        parse_quote! {<#ty as #crate_rename::TS>::OptionInnerType}
    };

    if field_attr.flatten {
        flattened_fields.push(quote!(<#ty as #crate_rename::TS>::inline_flattened()));
        dependencies.append_from(&ty);
        return Ok(());
    }

    let formatted_ty = field_attr
        .type_override
        .map(|t| quote!(#t))
        .unwrap_or_else(|| {
            if field_attr.inline {
                dependencies.append_from(&ty);
                quote!(<#ty as #crate_rename::TS>::inline())
            } else {
                dependencies.push(&ty);
                quote!(<#ty as #crate_rename::TS>::name())
            }
        });

    let field_name = to_ts_ident(field.ident.as_ref().unwrap());
    let name = match (field_attr.rename, rename_all) {
        (Some(rn), _) => rn,
        (None, Some(rn)) => rn.apply(&field_name),
        (None, None) => field_name,
    };
    let valid_name = raw_name_to_ts_field(name);

    // Start every doc string with a newline, because when other characters are in front, it is not "understood" by VSCode
    let docs = match field_attr.docs.is_empty() {
        true => "".to_string(),
        false => format!("\n{}", &field_attr.docs),
    };

    formatted_fields.push(quote! {
        format!("{}{}{}: {},", #docs, #valid_name, #optional_annotation, #formatted_ty)
    });

    Ok(())
}
