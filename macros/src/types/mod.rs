use syn::{Fields, Generics, Ident, ItemStruct, Result};

use crate::{attr::StructAttr, utils::to_ts_ident, DerivedTS};

mod r#enum;
mod generics;
mod named;
mod newtype;
mod tuple;
mod unit;

pub(crate) use r#enum::r#enum_def;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let attr = StructAttr::from_attrs(&s.attrs)?;

    type_def(&attr, &s.ident, &s.fields, &s.generics)
}

fn type_def(
    attr: &StructAttr,
    ident: &Ident,
    fields: &Fields,
    generics: &Generics,
) -> Result<DerivedTS> {
    let name = attr.rename.clone().unwrap_or_else(|| to_ts_ident(ident));
    match fields {
        Fields::Named(named) => match named.named.len() {
            0 => unit::empty_object(attr, &name),
            _ => named::named(attr, &name, named, generics),
        },
        Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => unit::empty_array(attr, &name),
            1 => newtype::newtype(attr, &name, unnamed, generics),
            _ => tuple::tuple(attr, &name, unnamed, generics),
        },
        Fields::Unit => unit::null(attr, &name),
    }
}

// TODO: This is a copy, because we do not have access to the `ts_rs` module or have a `utils` module.
/// Returns an unindented docstring that has a newline at the end if it has content.
fn format_docs(docs: &str) -> String {
    match docs.is_empty() {
        true => "".to_string(),
        false => {
            let lines = docs
                .lines()
                .map(|doc| format!(" *{doc}"))
                .collect::<Vec<_>>();
            format!("/**\n{}\n */\n", lines.join("\n"))
        }
    }
}
