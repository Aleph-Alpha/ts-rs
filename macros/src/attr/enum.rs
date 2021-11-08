use syn::{Attribute, Ident, Result};

use crate::{
    attr::{parse_assign_inflection, parse_assign_str, Inflection},
    utils::parse_attrs,
};

#[derive(Default)]
pub struct EnumAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
    tag: Option<String>,
    untag: bool,
    content: Option<String>,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeEnumAttr(EnumAttr);

#[derive(Copy, Clone)]
pub enum Tagged<'a> {
    Externally,
    Adjacently { tag: &'a str, content: &'a str },
    Internally { tag: &'a str },
    Untagged,
}

impl EnumAttr {
    pub fn tagged(&self) -> Result<Tagged<'_>> {
        match (self.untag, &self.tag, &self.content) {
            (false, None, None) => Ok(Tagged::Externally),
            (false, Some(tag), None) => Ok(Tagged::Internally { tag }),
            (false, Some(tag), Some(content)) => Ok(Tagged::Adjacently { tag, content }),
            (true, None, None) => Ok(Tagged::Untagged),
            (true, Some(_), None) => syn_err!("untagged cannot be used with tag"),
            (true, _, Some(_)) => syn_err!("untagged cannot be used with content"),
            (false, None, Some(_)) => syn_err!("content cannot be used without tag"),
        }
    }

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
        }: EnumAttr,
    ) {
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
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeEnumAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
        "tag" => out.0.tag = Some(parse_assign_str(input)?),
        "content" => out.0.content = Some(parse_assign_str(input)?),
        "untagged" => out.0.untag = true
    }
}
