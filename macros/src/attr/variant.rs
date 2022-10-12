use syn::{Attribute, Ident, Result};

use crate::{
    attr::{parse_assign_inflection, parse_assign_str, Inflection},
    utils::parse_attrs,
};

#[derive(Default)]
pub struct VariantAttr {
    pub rename: Option<String>,
    pub rename_all: Option<Inflection>,
    pub inline: bool,
    pub skip: bool,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeVariantAttr(VariantAttr);

impl VariantAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        #[cfg(feature = "serde-compat")]
        crate::utils::parse_serde_attrs::<SerdeVariantAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(
        &mut self,
        VariantAttr {
            rename,
            rename_all,
            inline,
            skip,
        }: VariantAttr,
    ) {
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
        self.inline = self.inline || inline;
        self.skip = self.skip || skip;
    }
}

impl_parse! {
    VariantAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_inflection(input)?),
        "inline" => out.inline = true,
        "skip" => out.skip = true,
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeVariantAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
        "skip" => out.0.skip = true,
        "skip_serializing" => out.0.skip = true,
        "skip_deserializing" => out.0.skip = true,
    }
}
