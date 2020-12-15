use crate::attr::{Inflection, parse_assign_string_lit};
use syn::{Attribute, Error, Result, Token, Ident};
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::ext::IdentExt;

#[derive(Default)]
pub struct StructAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
}

impl TryFrom<&Attribute> for StructAttr {
    type Error = Error;

    fn try_from(attr: &Attribute) -> Result<Self> {
        attr.parse_args()
    }
}


impl StructAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(StructAttr::try_from)
            .collect::<Result<Vec<StructAttr>>>()
            .map(|attrs| Self::merge(&attrs))
    }

    fn merge(attrs: &[StructAttr]) -> Self {
        let mut result = Self::default();
        for attr in attrs {
            result.rename = result.rename.or_else(|| attr.rename.clone());
            result.rename_all = result.rename_all.or_else(|| attr.rename_all);
        }
        result
    }
}

impl Parse for StructAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut out = StructAttr::default();
        loop {
            let key = input.call(Ident::parse_any)?;
            match &*key.to_string() {
                "rename" => out.rename = Some(parse_assign_string_lit(input)?),
                "rename_all" => out.rename_all = Some(parse_assign_string_lit(input).and_then(Inflection::try_from)?),
                _ => return Err(Error::new(input.span(), "unexpected key")),
            };

            match input.is_empty() {
                true => break,
                false => {
                    input.parse::<Token![,]>()?;
                }
            };
        }

        Ok(out)
    }
}
