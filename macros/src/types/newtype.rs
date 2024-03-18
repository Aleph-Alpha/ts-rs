use quote::quote;
use syn::{FieldsUnnamed, Result};

use crate::{
    attr::{FieldAttr, StructAttr},
    deps::Dependencies,
    DerivedTS,
};

pub(crate) fn newtype(attr: &StructAttr, name: &str, fields: &FieldsUnnamed) -> Result<DerivedTS> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to newtype structs");
    }
    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to newtype structs");
    }
    let inner = fields.unnamed.first().unwrap();
    let FieldAttr {
        type_as,
        type_override,
        rename: rename_inner,
        inline,
        skip,
        optional,
        flatten,
        docs: _,
    } = FieldAttr::from_attrs(&inner.attrs)?;

    match (&rename_inner, skip, optional.optional, flatten) {
        (Some(_), ..) => syn_err_spanned!(fields; "`rename` is not applicable to newtype fields"),
        (_, true, ..) => return super::unit::null(attr, name),
        (_, _, true, ..) => {
            syn_err_spanned!(fields; "`optional` is not applicable to newtype fields")
        }
        (_, _, _, true) => {
            syn_err_spanned!(fields; "`flatten` is not applicable to newtype fields")
        }
        _ => {}
    };

    if type_as.is_some() && type_override.is_some() {
        syn_err_spanned!(fields; "`type` is not compatible with `as`")
    }

    let inner_ty = type_as.as_ref().unwrap_or(&inner.ty).clone();

    let mut dependencies = Dependencies::default();

    match (type_override.is_none(), inline) {
        (false, _) => (),
        (true, true) => dependencies.append_from(&inner_ty),
        (true, false) => dependencies.push(&inner_ty),
    };

    let inline_def = match type_override {
        Some(ref o) => quote!(#o.to_owned()),
        None if inline => quote!(<#inner_ty as ::ts_rs::TS>::inline()),
        None => quote!(<#inner_ty as ::ts_rs::TS>::name()),
    };

    Ok(DerivedTS {
        inline: inline_def,
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
