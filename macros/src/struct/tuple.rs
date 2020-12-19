use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsUnnamed, ItemStruct, Result};

use crate::attr::{FieldAttr, StructAttr};
use crate::DerivedTS;

pub(crate) fn tuple(s: &ItemStruct, i: &FieldsUnnamed) -> Result<DerivedTS> {
    let StructAttr { rename_all, rename } = StructAttr::from_attrs(&s.attrs)?;
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to tuple structs");
    }

    let name = rename.unwrap_or_else(|| s.ident.to_string());

    let mut formatted_fields = Vec::new();
    let mut dependenciees = Vec::new();
    for field in &i.unnamed {
        format_field(&mut formatted_fields, &mut dependenciees, field)?;
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
                "export type {} = {};",
                #name,
                Self::inline(0)
            )
        },
        inline_flattened: None,
        name,
        dependencies: quote! {
            let mut dependencies = vec![];
            #( #dependenciees )*
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
        flatten,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(());
    }
    if rename.is_some() {
        syn_err!("`rename` is not applicable to tuple structs")
    }
    if flatten {
        syn_err!("`flatten` is not applicable to newtype fields")
    }

    formatted_fields.push(match &type_override {
        Some(o) => quote!(#o.to_owned()),
        None if inline => quote!(<#ty as ts_rs::TS>::inline(0)),
        None => quote!(<#ty as ts_rs::TS>::name()),
    });

    dependencies.push(match (inline, &type_override) {
        (_, Some(_)) => quote!(),
        (false, _) => quote! {
            dependencies.push((std::any::TypeId::of::<#ty>(), <#ty as ts_rs::TS>::name()));
        },
        (true, _) => quote! {
            dependencies.append(&mut (<#ty as ts_rs::TS>::dependencies()));
        },
    });

    Ok(())
}
