use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsUnnamed, Result};

use crate::attr::{FieldAttr, Inflection};
use crate::DerivedTS;

pub(crate) fn tuple(
    name: &str,
    rename_all: &Option<Inflection>,
    fields: &FieldsUnnamed,
) -> Result<DerivedTS> {
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to tuple structs");
    }

    let mut formatted_fields = Vec::new();
    let mut dependencies = Vec::new();
    for field in &fields.unnamed {
        format_field(&mut formatted_fields, &mut dependencies, field)?;
    }

    Ok(DerivedTS {
        inline: quote! {
            format!(
                "[{}]",
                vec![#(#formatted_fields),*].join(", ")
            )
        },
        decl: quote! {
            format!(
                "type {} = {};",
                #name,
                Self::inline(0)
            )
        },
        inline_flattened: None,
        name: name.to_owned(),
        dependencies: quote! {
            let mut dependencies = vec![];
            #( #dependencies )*
            dependencies
        },
    })
}

fn format_field(
    formatted_fields: &mut Vec<TokenStream>,
    dependencies: &mut Vec<TokenStream>,
    field: &Field,
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
        None if inline => quote!(<#ty as ts_rs::TS>::inline(0)),
        None => quote!(<#ty as ts_rs::TS>::name()),
    });

    dependencies.push(match (inline, &type_override) {
        (_, Some(_)) => quote!(),
        (false, _) => quote! {
            if <#ty as ts_rs::TS>::transparent() {
                dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());
            } else {
                dependencies.push((std::any::TypeId::of::<#ty>(), <#ty as ts_rs::TS>::name()));
            }
        },
        (true, _) => quote! {
            dependencies.append(&mut (<#ty as ts_rs::TS>::dependencies()));
        },
    });

    Ok(())
}
