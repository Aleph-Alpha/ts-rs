use super::parse_assign_string_lit;
use std::convert::TryFrom;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Ident, Result, Token};

#[derive(Default)]
pub struct FieldAttr {
    pub type_override: Option<String>,
    pub rename: Option<String>,
    pub inline: bool,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(FieldAttr::try_from)
            .collect::<Result<Vec<FieldAttr>>>()
            .map(|attrs| Self::merge(&attrs))
    }

    fn merge(attrs: &[FieldAttr]) -> Self {
        let mut result = Self::default();
        for attr in attrs {
            result.type_override = result.type_override.or_else(|| attr.type_override.clone());
            result.rename = result.rename.or_else(|| attr.rename.clone());
            result.inline = result.inline || attr.inline;
        }
        result
    }
}

impl TryFrom<&Attribute> for FieldAttr {
    type Error = Error;

    fn try_from(attr: &Attribute) -> Result<Self> {
        attr.parse_args()
    }
}

impl Parse for FieldAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut out = FieldAttr::default();
        loop {
            let key = input.call(Ident::parse_any)?;
            match &*key.to_string() {
                "type" => out.type_override = Some(parse_assign_string_lit(input)?),
                "rename" => out.rename = Some(parse_assign_string_lit(input)?),
                "inline" => out.inline = true,
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
