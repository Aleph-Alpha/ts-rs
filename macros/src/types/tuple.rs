use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, FieldsUnnamed, Generics, Result, Type};

use crate::{
    attr::{FieldAttr, StructAttr},
    deps::Dependencies,
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

    Ok(DerivedTS {
        generics: generics.clone(),
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
    })
}

fn format_field(
    formatted_fields: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    field: &Field,
    _generics: &Generics,
) -> Result<()> {
    let FieldAttr {
        type_as,
        type_override,
        rename,
        inline,
        skip,
        optional,
        flatten,
        docs: _,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(());
    }

    let ty = if let Some(ref type_as) = type_as {
        syn::parse_str::<Type>(&type_as.to_token_stream().to_string())?
    } else {
        field.ty.clone()
    };

    if type_as.is_some() && type_override.is_some() {
        syn_err_spanned!(field; "`type` is not compatible with `as`")
    }

    if rename.is_some() {
        syn_err_spanned!(field; "`rename` is not applicable to tuple structs")
    }

    if optional.optional {
        syn_err_spanned!(field; "`optional` is not applicable to tuple fields")
    }

    if flatten {
        syn_err_spanned!(field; "`flatten` is not applicable to tuple fields")
    }

    formatted_fields.push(match type_override {
        Some(ref o) => quote!(#o.to_owned()),
        None if inline => quote!(<#ty as ts_rs::TS>::inline()),
        None => quote!(<#ty as ts_rs::TS>::name()),
    });

    match (inline, type_override) {
        (_, Some(_)) => (),
        (false, _) => dependencies.push(&ty),
        (true, _) => dependencies.append_from(&ty),
    };

    Ok(())
}
