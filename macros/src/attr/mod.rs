use std::convert::TryFrom;

pub use field::*;
pub use r#enum::*;
pub use r#struct::*;
use syn::{
    parse::{Parse, ParseStream},
    Error, Lit, Result, Token,
};
pub use variant::*;

mod r#enum;
mod field;
mod r#struct;
mod variant;

#[derive(Copy, Clone, Debug)]
pub enum Inflection {
    Lower,
    Upper,
    Camel,
    Snake,
    Pascal,
    ScreamingSnake,
    Kebab,
}

impl Inflection {
    pub fn apply(self, string: &str) -> String {
        use inflector::Inflector;

        match self {
            Inflection::Lower => string.to_lowercase(),
            Inflection::Upper => string.to_uppercase(),
            Inflection::Camel => {
                let pascal = Inflection::apply(Inflection::Pascal, string);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            Inflection::Snake => string.to_snake_case(),
            Inflection::Pascal => {
                let mut s = String::with_capacity(string.len());

                let mut capitalize = true;
                for c in string.chars() {
                    if c == '_' {
                        capitalize = true;
                        continue;
                    } else if capitalize {
                        s.push(c.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        s.push(c.to_ascii_lowercase())
                    }
                }

                s
            }
            Inflection::ScreamingSnake => string.to_screaming_snake_case(),
            Inflection::Kebab => string.to_kebab_case(),
        }
    }
}

impl TryFrom<String> for Inflection {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(match &*value.to_lowercase().replace(['_', '-'], "") {
            "lowercase" => Self::Lower,
            "uppercase" => Self::Upper,
            "camelcase" => Self::Camel,
            "snakecase" => Self::Snake,
            "pascalcase" => Self::Pascal,
            "screamingsnakecase" => Self::ScreamingSnake,
            "kebabcase" => Self::Kebab,
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
