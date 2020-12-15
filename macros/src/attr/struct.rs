use std::convert::TryFrom;

use syn::{Attribute, Ident, Result};

use crate::attr::{parse_assign_str, Inflection};

#[derive(Default)]
pub struct StructAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
}

impl StructAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(StructAttr::try_from)
            .collect::<Result<Vec<StructAttr>>>()
            .map(|attrs| Self::merge(&attrs))
    }

    fn merge(attrs: &[StructAttr]) -> Self {
        let mut result = Self::default();
        for attr in attrs {
            result.rename = result.rename.or_else(|| attr.rename.clone());
            result.rename_all = result.rename_all.or_else(|| attr.rename_all);
        }
        result
    }
}

impl_parse! {
    StructAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
    }
}
