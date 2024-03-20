use std::{collections::HashMap, convert::TryFrom};

use proc_macro2::TokenStream;
use syn::{Attribute, Ident, Result, Type, WherePredicate};

use super::{parse_assign_from_str, parse_bound};
use crate::{
    attr::{parse_assign_str, parse_concrete, Inflection, VariantAttr},
    utils::{parse_attrs, parse_docs},
};

#[derive(Default, Clone)]
pub struct StructAttr {
    pub crate_rename: Option<TokenStream>,
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
    pub export_to: Option<String>,
    pub export: bool,
    pub tag: Option<String>,
    pub docs: String,
    pub concrete: HashMap<Ident, Type>,
    pub bound: Option<Vec<WherePredicate>>,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeStructAttr(StructAttr);

impl StructAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));

        let docs = parse_docs(attrs)?;
        result.docs = docs;

        #[cfg(feature = "serde-compat")]
        crate::utils::parse_serde_attrs::<SerdeStructAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(
        &mut self,
        StructAttr {
            crate_rename,
            rename_all,
            rename,
            export,
            export_to,
            tag,
            docs,
            concrete,
            bound,
        }: StructAttr,
    ) {
        self.crate_rename = self.crate_rename.take().or(crate_rename);
        self.rename = self.rename.take().or(rename);
        self.rename_all = self.rename_all.take().or(rename_all);
        self.export_to = self.export_to.take().or(export_to);
        self.export = self.export || export;
        self.tag = self.tag.take().or(tag);
        self.docs = docs;
        self.concrete.extend(concrete);
        self.bound = self
            .bound
            .take()
            .map(|b| {
                b.into_iter()
                    .chain(bound.clone().unwrap_or_default())
                    .collect()
            })
            .or(bound);
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
        "crate" => out.crate_rename = Some(parse_assign_from_str(input)?),
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
        "tag" => out.tag = Some(parse_assign_str(input)?),
        "export" => out.export = true,
        "export_to" => out.export_to = Some(parse_assign_str(input)?),
        "concrete" => out.concrete = parse_concrete(input)?,
        "bound" => out.bound = Some(parse_bound(input)?),
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeStructAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "rename_all" => out.0.rename_all = Some(parse_assign_str(input).and_then(Inflection::try_from)?),
        "tag" => out.0.tag = Some(parse_assign_str(input)?),
        "bound" => out.0.bound = Some(parse_bound(input)?),
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
