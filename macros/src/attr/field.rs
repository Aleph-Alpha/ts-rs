use super::parse_assign_str;
use std::convert::TryFrom;
use syn::{Attribute, Ident, Result};

#[derive(Default)]
pub struct FieldAttr {
    pub type_override: Option<String>,
    pub rename: Option<String>,
    pub inline: bool,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(FieldAttr::try_from)
            .collect::<Result<Vec<FieldAttr>>>()
            .map(|attrs| Self::merge(&attrs))
    }

    fn merge(attrs: &[FieldAttr]) -> Self {
        let mut result = Self::default();
        for attr in attrs {
            result.type_override = result.type_override.or_else(|| attr.type_override.clone());
            result.rename = result.rename.or_else(|| attr.rename.clone());
            result.inline = result.inline || attr.inline;
        }
        result
    }
}

impl_parse! {
    FieldAttr(input, out) {
        "type" => out.type_override = Some(parse_assign_str(input)?),
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "inline" => out.inline = true,
    }
}
