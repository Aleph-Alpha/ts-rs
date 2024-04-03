use quote::quote;
use syn::{FieldsUnnamed, Result};

use crate::{
    attr::{Attr, ContainerAttr, FieldAttr, StructAttr},
    deps::Dependencies,
    DerivedTS,
};

pub(crate) fn newtype(attr: &StructAttr, name: &str, fields: &FieldsUnnamed) -> Result<DerivedTS> {
    let inner = fields.unnamed.first().unwrap();

    let field_attr = FieldAttr::from_attrs(&inner.attrs)?;
    field_attr.assert_validity(inner)?;

    let FieldAttr {
        type_as,
        type_override,
        inline,
        skip,
        docs: _,

        #[cfg(feature = "serde-compat")]
        using_serde_with,
        ..
    } = field_attr;

    let crate_rename = attr.crate_rename();

    if skip {
        return super::unit::null(attr, name);
    }

    let inner_ty = type_as.as_ref().unwrap_or(&inner.ty).clone();

    let mut dependencies = Dependencies::new(crate_rename.clone());

    match (&type_override, inline) {
        (Some(_), _) => (),
        (None, true) => dependencies.append_from(&inner_ty),
        (None, false) => dependencies.push(&inner_ty),
    };

    let inline_def = match type_override {
        Some(ref o) => quote!(#o.to_owned()),
        None if inline => quote!(<#inner_ty as #crate_rename::TS>::inline()),
        None => quote!(<#inner_ty as #crate_rename::TS>::name()),
    };

    Ok(DerivedTS {
        crate_rename,
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
