use std::convert::TryFrom;

use syn::{parse::Parse, parse::ParseStream, Error, Lit, Result, Token};

pub use field::*;
pub use r#enum::*;
pub use r#struct::*;

mod r#enum;
mod field;
mod r#struct;

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
            Inflection::ScreamingSnake => string.to_screaming_snake_case(),
        }
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
            _ => syn_err!("invalid inflection: '{}'", value),
        })
    }
}

fn parse_assign_str(input: ParseStream) -> Result<String> {
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => Ok(string.value()),
        other => Err(Error::new(other.span(), "expected string")),
    }
}

fn parse_assign_inflection(input: ParseStream) -> Result<Inflection> {
    parse_assign_str(input).and_then(Inflection::try_from)
}
