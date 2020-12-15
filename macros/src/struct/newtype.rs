use crate::attr::FieldAttr;
use crate::DerivedTS;
use quote::quote;
use syn::{FieldsUnnamed, ItemStruct, Result};

pub(crate) fn newtype(s: &ItemStruct, i: &FieldsUnnamed) -> Result<DerivedTS> {
    let inner = i.unnamed.first().unwrap();
    let FieldAttr {
        type_override,
        rename,
        inline,
    } = FieldAttr::from_attrs(&inner.attrs)?;

    if rename.is_some() {
        syn_err!("`rename` is not applicable to tuple structs")
    }

    let name = s.ident.to_string();
    let inner_ty = &inner.ty;
    let inline_def = match type_override {
        Some(o) => quote!(#o.into()),
        None => quote!(<#inner_ty as ts_rs::TS>::format(0, #inline)),
    };
    Ok(DerivedTS {
        format: Default::default(),
        decl: quote!(format!("export type {} = {};", #name, #inline_def)),
        name,
    })
}
