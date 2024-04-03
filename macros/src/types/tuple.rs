use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsUnnamed, Path, Result};

use crate::{
    attr::{Attr, ContainerAttr, FieldAttr, StructAttr},
    deps::Dependencies,
    DerivedTS,
};

pub(crate) fn tuple(attr: &StructAttr, name: &str, fields: &FieldsUnnamed) -> Result<DerivedTS> {
    let crate_rename = attr.crate_rename();
    let mut formatted_fields = Vec::new();
    let mut dependencies = Dependencies::new(crate_rename.clone());
    for field in &fields.unnamed {
        format_field(
            &crate_rename,
            &mut formatted_fields,
            &mut dependencies,
            field,
        )?;
    }

    Ok(DerivedTS {
        crate_rename,
        inline: quote! {
            format!(
                "[{}]",
                [#(#formatted_fields),*].join(", ")
            )
        },
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies,
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

fn format_field(
    crate_rename: &Path,
    formatted_fields: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    field: &Field,
) -> Result<()> {
    let field_attr = FieldAttr::from_attrs(&field.attrs)?;
    field_attr.assert_validity(field)?;

    let FieldAttr {
        type_as,
        type_override,
        rename: _,
        inline,
        skip,
        optional: _,
        flatten: _,
        docs: _,
    } = field_attr;

    if skip {
        return Ok(());
    }

    let ty = type_as.as_ref().unwrap_or(&field.ty).clone();

    formatted_fields.push(match type_override {
        Some(ref o) => quote!(#o.to_owned()),
        None if inline => quote!(<#ty as #crate_rename::TS>::inline()),
        None => quote!(<#ty as #crate_rename::TS>::name()),
    });

    match (inline, type_override) {
        (_, Some(_)) => (),
        (false, _) => dependencies.push(&ty),
        (true, _) => dependencies.append_from(&ty),
    };

    Ok(())
}
