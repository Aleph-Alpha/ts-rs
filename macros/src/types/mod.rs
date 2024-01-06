use syn::{Attribute, Fields, Generics, Ident, ItemStruct, Result};

use crate::{
    attr::StructAttr,
    utils::{self, to_ts_ident},
    DerivedTS,
};

mod r#enum;
mod generics;
mod named;
mod newtype;
mod tuple;
mod unit;

pub(crate) use r#enum::r#enum_def;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let attr = StructAttr::from_attrs(&s.attrs)?;

    type_def(&attr, &s.attrs, &s.ident, &s.fields, &s.generics)
}

fn type_def(
    attr: &StructAttr,
    attrs: &Vec<Attribute>,
    ident: &Ident,
    fields: &Fields,
    generics: &Generics,
) -> Result<DerivedTS> {
    let name = attr.rename.clone().unwrap_or_else(|| to_ts_ident(ident));
    let docs = utils::get_docs_from_attributes(attrs);
    match fields {
        Fields::Named(named) => match named.named.len() {
            0 => unit::empty_object(attr, &name, &docs),
            _ => named::named(attr, &name, &docs, named, generics),
        },
        Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => unit::empty_array(attr, &name, &docs),
            1 => newtype::newtype(attr, &name, &docs, unnamed, generics),
            _ => tuple::tuple(attr, &name, &docs, unnamed, generics),
        },
        Fields::Unit => unit::null(attr, &name, &docs),
    }
}
