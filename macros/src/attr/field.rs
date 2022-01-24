use syn::{Attribute, Ident, Result};

use super::parse_assign_str;
use crate::utils::parse_attrs;

#[derive(Default)]
pub struct FieldAttr {
    pub type_override: Option<String>,
    pub rename: Option<String>,
    pub inline: bool,
    pub skip: bool,
    pub optional: bool,
    pub flatten: bool,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeFieldAttr(FieldAttr);

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        #[cfg(feature = "serde-compat")]
        crate::utils::parse_serde_attrs::<SerdeFieldAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(
        &mut self,
        FieldAttr {
            type_override,
            rename,
            inline,
            skip,
            optional,
            flatten,
        }: FieldAttr,
    ) {
        self.rename = self.rename.take().or(rename);
        self.type_override = self.type_override.take().or(type_override);
        self.inline = self.inline || inline;
        self.skip = self.skip || skip;
        self.optional |= optional;
        self.flatten |= flatten;
    }
}

impl_parse! {
    FieldAttr(input, out) {
        "type" => out.type_override = Some(parse_assign_str(input)?),
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "inline" => out.inline = true,
        "skip" => out.skip = true,
        "optional" => out.optional = true,
        "flatten" => out.flatten = true,
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeFieldAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "skip" => out.0.skip = true,
        "skip_serializing" => out.0.skip = true,
        "skip_deserializing" => out.0.skip = true,
        "skip_serializing_if" => out.0.optional = parse_assign_str(input)? == *"Option::is_none",
        "flatten" => out.0.flatten = true,
        // parse #[serde(default)] to not emit a warning
        "default" => {
            use syn::Token;
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                parse_assign_str(input)?;
            }
        },
    }
}
