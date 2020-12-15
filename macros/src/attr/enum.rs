use std::convert::TryFrom;

use syn::{Attribute, Error, Ident, Result, Token};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};

use crate::attr::parse_assign_string_lit;

#[derive(Default)]
pub struct EnumAttr {
    pub rename_all: Option<Inflection>,
    pub rename: Option<String>,
}

#[derive(Copy, Clone, Debug)]
pub enum Inflection {
    Lower,
    Upper,
    Camel,
    Snake,
    Pascal,
    ScreamingSnake,
}

impl Inflection {
    pub fn apply(self, string: &str) -> String {
        use inflector::Inflector;

        match self {
            Inflection::Lower => string.to_lowercase(),
            Inflection::Upper => string.to_uppercase(),
            Inflection::Camel => string.to_camel_case(),
            Inflection::Snake => string.to_snake_case(),
            Inflection::Pascal => string.to_pascal_case(),
            Inflection::ScreamingSnake => string.to_screaming_snake_case()
        }
    }
}

impl TryFrom<&Attribute> for EnumAttr {
    type Error = Error;

    fn try_from(attr: &Attribute) -> Result<Self> {
        attr.parse_args()
    }
}

impl TryFrom<String> for Inflection {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(match &*value.to_lowercase() {
            "lowercase" => Self::Lower,
            "uppercase" => Self::Upper,
            "camelcase" => Self::Camel,
            "snake_case" | "snakecase" => Self::Snake,
            "pascalcase" => Self::Pascal,
            "screaming_snake_case" | "screamingsnakecase" => Self::ScreamingSnake,
            _ => syn_err!("invalid inflection: '{}'", value)
        })
    }
}

impl EnumAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        attrs
            .iter()
            .filter(|a| a.path.is_ident("ts"))
            .map(EnumAttr::try_from)
            .collect::<Result<Vec<EnumAttr>>>()
            .map(|attrs| Self::merge(&attrs))
    }

    fn merge(attrs: &[EnumAttr]) -> Self {
        let mut result = Self::default();
        for attr in attrs {
            result.rename = result.rename.or_else(|| attr.rename.clone());
            result.rename_all = result.rename_all.or_else(|| attr.rename_all);
        }
        result
    }
}

impl Parse for EnumAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut out = EnumAttr::default();
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
