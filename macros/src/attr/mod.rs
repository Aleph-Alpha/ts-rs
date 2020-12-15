use std::convert::TryFrom;

use syn::{parse::Parse, parse::ParseStream, Attribute, Error, Ident, Lit, Result, Token};

pub use field::*;

mod field;

#[derive(Default)]
struct TypeAttr {
    rename: Option<String>,
}

impl TryFrom<&Attribute> for FieldAttr {
    type Error = Error;

    fn try_from(attr: &Attribute) -> Result<Self> {
        attr.parse_args()
    }
}

impl TryFrom<&Attribute> for TypeAttr {
    type Error = Error;

    fn try_from(attr: &Attribute) -> Result<Self> {
        attr.parse_args()
    }
}

impl Parse for TypeAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut out = TypeAttr::default();
        while !input.is_empty() {
            let key = Ident::parse(input)?;
            match &*key.to_string() {
                "rename" => out.rename = Some(parse_assign_string_lit(input)?),
                _ => return Err(Error::new(input.span(), "unexpected key")),
            };
        }

        Ok(out)
    }
}

fn parse_assign_string_lit(input: ParseStream) -> Result<String> {
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => Ok(string.value()),
        other => Err(Error::new(other.span(), "expected string")),
    }
}
