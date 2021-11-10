use syn::{Fields, Generics, ItemStruct, Result};

use crate::{
    attr::{Inflection, StructAttr},
    utils::to_ts_ident,
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
    let StructAttr {
        rename_all,
        rename,
        export,
    } = StructAttr::from_attrs(&s.attrs)?;
    let name = rename.unwrap_or_else(|| to_ts_ident(&s.ident));

    type_def(&name, &rename_all, &s.fields, &s.generics, export)
}

fn type_def(
    name: &str,
    rename_all: &Option<Inflection>,
    fields: &Fields,
    generics: &Generics,
    export: Option<Option<String>>,
) -> Result<DerivedTS> {
    match fields {
        Fields::Named(named) => match named.named.len() {
            0 => unit::unit(name, rename_all, export),
            _ => named::named(name, rename_all, named, generics, export),
        },
        Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => unit::unit(name, rename_all, export),
            1 => newtype::newtype(name, rename_all, unnamed, generics, export),
            _ => tuple::tuple(name, rename_all, unnamed, generics, export),
        },
        Fields::Unit => unit::unit(name, rename_all, export),
    }
}
