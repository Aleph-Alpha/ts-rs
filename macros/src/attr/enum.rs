use syn::{Attribute, Ident, Result};

use crate::attr::{parse_assign_inflection, parse_assign_str, Inflection};
use crate::utils::parse_attrs;

#[derive(Default)]
pub struct EnumAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
    pub tag: Option<String>,
    pub untag: bool,
    pub content: Option<String>,
    pub r#type: Option<String>
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeEnumAttr(EnumAttr);

impl EnumAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        #[cfg(feature = "serde-compat")]
        crate::utils::parse_serde_attrs::<SerdeEnumAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(
        &mut self,
        EnumAttr {
            rename_all,
            rename,
            tag,
            content,
            untag,
            r#type
        }: EnumAttr,
    ) {
        self.r#type = self.r#type.take().or(r#type);
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
        self.tag = self.tag.take().or(tag);
        self.untag = self.untag || untag;
        self.content = self.content.take().or(content);
    }
}

impl_parse! {
    EnumAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_inflection(input)?),
        "type" => out.r#type = Some(parse_assign_str(input)?)
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeEnumAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
        "tag" => out.0.tag = Some(parse_assign_str(input)?),
        "content" => out.0.content = Some(parse_assign_str(input)?),
        "untagged" => out.0.untag = true,
        "type" => out.0.r#type = Some(parse_assign_str(input)?)
    }
}
