use syn::{Fields, ItemStruct, Result};

use crate::DerivedTS;

mod named;
mod newtype;
mod tuple;
mod unit;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    match &s.fields {
        Fields::Named(named) => named::named(s, &named),
        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => newtype::newtype(s, &unnamed),
        Fields::Unnamed(unnamed) => tuple::tuple(s, &unnamed),
        Fields::Unit => unit::unit(s),
    }
}
