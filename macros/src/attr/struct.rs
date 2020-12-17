use std::convert::TryFrom;

use syn::{Attribute, Ident, Result};

use crate::attr::{parse_assign_str, Inflection};
use crate::utils::{parse_attrs, parse_serde_attrs};

#[derive(Default)]
pub struct StructAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeStructAttr(StructAttr);

impl StructAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        #[cfg(feature = "serde-compat")]
        parse_serde_attrs::<SerdeStructAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(&mut self, StructAttr { rename_all, rename }: StructAttr) {
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
    }
}

impl_parse! {
    StructAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeStructAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
    }
}
