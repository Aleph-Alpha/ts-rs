use crate::attr::{FieldAttr, StructAttr};
use crate::DerivedTS;
use quote::quote;
use syn::{FieldsUnnamed, ItemStruct, Result};

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
        flatten
    } = FieldAttr::from_attrs(&inner.attrs)?;

    if rename_inner.is_some() {
        syn_err!("`rename` is not applicable to newtype fields")
    }
    if skip {
        syn_err!("`skip` is not applicable to newtype fields")
    }
    if flatten {
        syn_err!("`flatten` is not applicable to newtype fields")
    }

    let name = rename_outer.unwrap_or_else(|| s.ident.to_string());
    let inner_ty = &inner.ty;
    let inline_def = match type_override {
        Some(o) => quote!(#o.into()),
        None => quote!(<#inner_ty as ts_rs::TS>::format(0, #inline)),
    };
    Ok(DerivedTS {
        format: Default::default(),
        decl: quote!(format!("export type {} = {};", #name, #inline_def)),
        flatten: None,
        name,
    })
}
