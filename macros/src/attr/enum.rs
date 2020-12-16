use std::convert::TryFrom;

use crate::attr::{parse_assign_inflection, parse_assign_str, Inflection};
use syn::{Attribute, Ident, Result};

#[derive(Default)]
pub struct EnumAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeEnumAttr(EnumAttr);

impl EnumAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(EnumAttr::try_from)
            .collect::<Result<Vec<EnumAttr>>>()?
            .into_iter()
            .for_each(|a| result.merge(a));

        #[cfg(feature = "serde-compat")]
        {
            attrs
                .iter()
                .filter(|a| a.path.is_ident("serde"))
                .flat_map(|attr| match SerdeEnumAttr::try_from(attr) {
                    Ok(attr) => Some(attr),
                    Err(_) => {
                        use quote::ToTokens;
                        crate::utils::print_warning(
                            "failed to parse serde attribute",
                            format!("{}", attr.to_token_stream()),
                            "ts-rs failed to parse this attribute. It will be ignored.",
                        )
                        .unwrap();
                        None
                    }
                })
                .for_each(|a| result.merge(a.0));
        }

        Ok(result)
    }

    fn merge(&mut self, EnumAttr { rename_all, rename }: EnumAttr) {
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
    }
}

impl_parse! {
    EnumAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_inflection(input)?),
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeEnumAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
    }
}
