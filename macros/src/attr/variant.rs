use syn::{Attribute, Fields, Ident, Result, Variant};

use super::Attr;
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
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = parse_attrs(attrs)?.fold(Self::default(), |acc, cur| acc.merge(cur));
        #[cfg(feature = "serde-compat")]
        if !result.skip {
            result = crate::utils::parse_serde_attrs::<SerdeVariantAttr>(attrs)
                .fold(result, |acc, cur| acc.merge(cur.0));
        }
        Ok(result)
    }
}

impl Attr for VariantAttr {
    type Item = Variant;

    fn merge(self, other: Self) -> Self {
        Self {
            rename: self.rename.or(other.rename),
            rename_all: self.rename_all.or(other.rename_all),
            inline: self.inline || other.inline,
            skip: self.skip || other.skip,
            untagged: self.untagged || other.untagged,
        }
    }

    fn assert_validity(&self, item: &Self::Item) -> Result<()> {
        if !matches!(item.fields, Fields::Named(_)) && self.rename_all.is_some() {
            syn_err_spanned!(
                item;
                "`rename_all` is not applicable to unit or tuple variants"
            )
        }

        Ok(())
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
