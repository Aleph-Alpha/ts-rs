use quote::quote;
use syn::{FieldsUnnamed, ItemStruct, Result};

use crate::attr::{FieldAttr, StructAttr};
use crate::DerivedTS;

pub(crate) fn newtype(s: &ItemStruct, i: &FieldsUnnamed) -> Result<DerivedTS> {
    let StructAttr {
        rename_all,
        rename: rename_outer,
    } = StructAttr::from_attrs(&s.attrs)?;
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to newtype structs");
    }
    let inner = i.unnamed.first().unwrap();
    let FieldAttr {
        type_override,
        rename: rename_inner,
        inline,
        skip,
        flatten,
    } = FieldAttr::from_attrs(&inner.attrs)?;

    match (&rename_inner, skip, flatten) {
        (Some(_), _, _) => syn_err!("`rename` is not applicable to newtype fields"),
        (_, true, _) => syn_err!("`skip` is not applicable to newtype fields"),
        (_, _, true) => syn_err!("`flatten` is not applicable to newtype fields"),
        _ => {}
    };

    let name = rename_outer.unwrap_or_else(|| s.ident.to_string());
    let inner_ty = &inner.ty;
    let inline_def = match type_override {
        Some(o) => quote!(#o.into()),
        None => quote!(<#inner_ty as ts_rs::TS>::format(0, #inline)),
    };
    Ok(DerivedTS {
        decl: quote!(format!("export type {} = {};", #name, #inline_def)),
        format: inline_def,
        flatten: None,
        name,
    })
}
