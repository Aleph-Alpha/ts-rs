use syn::{Fields, ItemStruct, Result};

use crate::{
    attr::{Attr, StructAttr},
    DerivedTS,
};

mod r#enum;
mod named;
mod newtype;
mod tuple;
mod type_as;
mod type_override;
mod unit;

pub(crate) use r#enum::r#enum_def;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let attr = StructAttr::from_attrs(&s.attrs)?;

    type_def(&attr, &s.ident.to_string(), &s.fields)
}

fn type_def(attr: &StructAttr, ident: &str, fields: &Fields) -> Result<DerivedTS> {
    attr.assert_validity(fields)?;

    let name = attr
        .rename
        .clone()
        .unwrap_or_else(|| ident.trim_start_matches("r#").to_owned());
    if let Some(attr_type_override) = &attr.type_override {
        return type_override::type_override_struct(attr, &name, attr_type_override);
    }
    if let Some(attr_type_as) = &attr.type_as {
        return type_as::type_as_struct(attr, &name, attr_type_as);
    }

    match fields {
        Fields::Named(named) => match named.named.len() {
            0 => unit::empty_object(attr, &name),
            _ => named::named(attr, &name, named),
        },
        Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => unit::empty_array(attr, &name),
            1 => newtype::newtype(attr, &name, unnamed),
            _ => tuple::tuple(attr, &name, unnamed),
        },
        Fields::Unit => unit::null(attr, &name),
    }
}
