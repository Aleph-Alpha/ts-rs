use syn::{Attribute, Ident, Result};

use super::EnumAttr;
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
    pub untagged: bool,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeVariantAttr(VariantAttr);

impl VariantAttr {
    pub fn new(attrs: &[Attribute], enum_attr: &EnumAttr) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        result.rename_all = result.rename_all.or(enum_attr.rename_all_fields);
        #[cfg(feature = "serde-compat")]
        if !result.skip {
            crate::utils::parse_serde_attrs::<SerdeVariantAttr>(attrs)
                .for_each(|a| result.merge(a.0));
        }
        Ok(result)
    }

    fn merge(
        &mut self,
        VariantAttr {
            rename,
            rename_all,
            inline,
            skip,
            untagged,
        }: VariantAttr,
    ) {
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
        self.inline = self.inline || inline;
        self.skip = self.skip || skip;
        self.untagged = self.untagged || untagged
    }
}

impl_parse! {
    VariantAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_inflection(input)?),
        "inline" => out.inline = true,
        "skip" => out.skip = true,
        "untagged" => out.untagged = true,
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeVariantAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
        "skip" => out.0.skip = true,
        "untagged" => out.0.untagged = true,
    }
}
