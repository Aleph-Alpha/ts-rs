use syn::{Fields, Ident, ItemStruct, Result};

use crate::{attr::StructAttr, utils::to_ts_ident, DerivedTS};

mod r#enum;
mod named;
mod newtype;
mod tuple;
mod type_override;
mod unit;

pub(crate) use r#enum::r#enum_def;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let attr = StructAttr::from_attrs(&s.attrs)?;

    type_def(&attr, &s.ident, &s.fields)
}

fn type_def(attr: &StructAttr, ident: &Ident, fields: &Fields) -> Result<DerivedTS> {
    let name = attr.rename.clone().unwrap_or_else(|| to_ts_ident(ident));
    if let Some(t_o) = &attr.type_override {
        return type_override::type_override_struct(attr, &name, t_o);
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
