use syn::{Attribute, Fields, Ident, Result, Variant};

use super::{Attr, Serde};
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

impl VariantAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = parse_attrs::<Self>(attrs)?;
        #[cfg(feature = "serde-compat")]
        if !result.skip {
            let serde_attr = crate::utils::parse_serde_attrs::<VariantAttr>(attrs);
            result.merge_with_serde(serde_attr);
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

    #[cfg(feature = "serde-compat")]
    fn merge_with_serde(&mut self, serde: Serde<Self>) {
        self.rename = self.rename.take().or(serde.0.rename);
        self.rename_all = self.rename_all.take().or(serde.0.rename_all);
        self.skip = self.skip || serde.0.skip;
        self.untagged = self.untagged || serde.0.untagged;
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
    Serde<VariantAttr>(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
        "skip" => out.0.skip = true,
        "untagged" => out.0.untagged = true,
    }
}
