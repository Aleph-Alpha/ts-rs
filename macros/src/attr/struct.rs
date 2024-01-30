use std::convert::TryFrom;

use syn::{Attribute, Ident, Result, Path};

use crate::{
    attr::{parse_assign_str, Inflection, VariantAttr},
    utils::parse_attrs,
};

#[derive(Default, Clone)]
pub struct StructAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
    pub export_to: Option<String>,
    pub export: bool,
    pub tag: Option<String>,
    pub unit_type: Option<Path>,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeStructAttr(StructAttr);

impl StructAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        #[cfg(feature = "serde-compat")]
        crate::utils::parse_serde_attrs::<SerdeStructAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(
        &mut self,
        StructAttr {
            rename_all,
            rename,
            export,
            export_to,
            tag,
            unit_type,
        }: StructAttr,
    ) {
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
        self.export_to = self.export_to.take().or(export_to);
        self.export = self.export || export;
        self.tag = self.tag.take().or(tag);
        self.unit_type = self.unit_type.take().or(unit_type);
    }
}

impl From<VariantAttr> for StructAttr {
    fn from(
        VariantAttr {
            rename, rename_all, ..
        }: VariantAttr,
    ) -> Self {
        Self {
            rename,
            rename_all,
            // inline and skip are not supported on StructAttr
            ..Self::default()
        }
    }
}

impl_parse! {
    StructAttr(input, out) {
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
        "unit_type" => out.unit_type = Some(
            parse_assign_str(input).and_then(|x| syn::parse_str::<Path>(&x))?
        ),
        "export" => out.export = true,
        "export_to" => out.export_to = Some(parse_assign_str(input)?)
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeStructAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
        "tag" => out.0.tag = Some(parse_assign_str(input)?),
        // parse #[serde(default)] to not emit a warning
        "deny_unknown_fields" | "default" => {
            use syn::Token;
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                parse_assign_str(input)?;
            }
        },
    }
}
