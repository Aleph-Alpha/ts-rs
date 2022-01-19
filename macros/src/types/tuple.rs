use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsUnnamed, Generics, Result};

use crate::{
    attr::{FieldAttr, StructAttr},
    deps::Dependencies,
    types::generics::{format_generics, format_type},
    DerivedTS,
};

pub(crate) fn tuple(
    attr: &StructAttr,
    name: &str,
    fields: &FieldsUnnamed,
    generics: &Generics,
) -> Result<DerivedTS> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to tuple structs");
    }
    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to tuple structs");
    }

    let mut formatted_fields = Vec::new();
    let mut dependencies = Dependencies::default();
    for field in &fields.unnamed {
        format_field(&mut formatted_fields, &mut dependencies, field, generics)?;
    }

    let generic_args = format_generics(&mut dependencies, generics);
    Ok(DerivedTS {
        inline: quote! {
            format!(
                "[{}]",
                vec![#(#formatted_fields),*].join(", ")
            )
        },
        decl: quote! {
            format!(
                "type {}{} = {};",
                #name,
                #generic_args,
                Self::inline()
            )
        },
        inline_flattened: None,
        name: name.to_owned(),
        dependencies,
        export: attr.export,
        export_to: attr.export_to.clone(),
    })
}

fn format_field(
    formatted_fields: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    field: &Field,
    generics: &Generics,
) -> Result<()> {
    let ty = &field.ty;
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip,
        optional,
        flatten,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(());
    }
    if rename.is_some() {
        syn_err!("`rename` is not applicable to tuple structs")
    }
    if optional {
        syn_err!("`optional` is not applicable to tuple fields")
    }
    if flatten {
        syn_err!("`flatten` is not applicable to tuple fields")
    }

    formatted_fields.push(match &type_override {
        Some(o) => quote!(#o.to_owned()),
        None if inline => quote!(<#ty as ts_rs::TS>::inline()),
        None => format_type(ty, dependencies, generics),
    });

    match (inline, &type_override) {
        (_, Some(_)) => (),
        (false, _) => {
            dependencies.push_or_append_from(ty);
        }
        (true, _) => {
            dependencies.append_from(ty);
        }
    };

    Ok(())
}
