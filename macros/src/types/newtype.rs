use quote::quote;
use syn::{FieldsUnnamed, Generics, Result};

use crate::{
    attr::{FieldAttr, Inflection},
    deps::Dependencies,
    types::generics::format_type,
    DerivedTS,
};

pub(crate) fn newtype(
    name: &str,
    rename_all: &Option<Inflection>,
    fields: &FieldsUnnamed,
    generics: &Generics,
) -> Result<DerivedTS> {
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to newtype structs");
    }
    let inner = fields.unnamed.first().unwrap();
    let FieldAttr {
        type_override,
        rename: rename_inner,
        inline,
        skip,
        optional,
        flatten,
    } = FieldAttr::from_attrs(&inner.attrs)?;

    match (&rename_inner, skip, optional, flatten) {
        (Some(_), ..) => syn_err!("`rename` is not applicable to newtype fields"),
        (_, true, ..) => syn_err!("`skip` is not applicable to newtype fields"),
        (_, _, true, ..) => syn_err!("`optional` is not applicable to newtype fields"),
        (_, _, _, true) => syn_err!("`flatten` is not applicable to newtype fields"),
        _ => {}
    };

    let inner_ty = &inner.ty;
    let mut dependencies = Dependencies::default();
    match (inline, &type_override) {
        (_, Some(_)) => (),
        (true, _) => dependencies.append_from(inner_ty),
        (false, _) => dependencies.push_or_append_from(inner_ty),
    };
    let inline_def = match &type_override {
        Some(o) => quote!(#o),
        None if inline => quote!(<#inner_ty as ts_rs::TS>::inline()),
        None => format_type(inner_ty, &mut dependencies, generics),
    };

    Ok(DerivedTS {
        decl: quote!(format!("type {} = {};", #name, #inline_def)),
        inline: inline_def,
        inline_flattened: None,
        name: name.to_owned(),
        dependencies,
    })
}
