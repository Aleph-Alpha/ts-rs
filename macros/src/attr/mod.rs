use std::collections::HashMap;

pub use field::*;
pub use r#enum::*;
pub use r#fn::*;
pub use r#struct::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Lit, Path, Result, Token, WherePredicate,
};
pub use variant::*;

mod r#enum;
mod field;
mod r#fn;
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
    ScreamingKebab,
}

pub(super) trait Attr: Default {
    type Item;

    fn merge(self, other: Self) -> Self;
    fn assert_validity(&self, item: &Self::Item) -> Result<()>;
}

pub(super) trait ContainerAttr: Attr {
    fn crate_rename(&self) -> Path;
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub(super) struct Serde<T>(pub T)
where
    T: Attr;

#[cfg(feature = "serde-compat")]
impl<T> Serde<T>
where
    T: Attr,
{
    pub fn merge(self, other: Self) -> Self {
        Self(self.0.merge(other.0))
    }
}

impl Inflection {
    pub fn apply(self, string: &str) -> String {
        match self {
            Inflection::Lower => string.to_lowercase(),
            Inflection::Upper => string.to_uppercase(),
            Inflection::Camel => {
                let pascal = Inflection::apply(Inflection::Pascal, string);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            Inflection::Snake => {
                let mut s = String::new();

                for (i, ch) in string.char_indices() {
                    if ch.is_uppercase() && i != 0 {
                        s.push('_');
                    }
                    s.push(ch.to_ascii_lowercase());
                }

                s
            }
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
                        s.push(c)
                    }
                }

                s
            }
            Inflection::ScreamingSnake => Self::Snake.apply(string).to_ascii_uppercase(),
            Inflection::Kebab => Self::Snake.apply(string).replace('_', "-"),
            Inflection::ScreamingKebab => Self::Kebab.apply(string).to_ascii_uppercase(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Lower => "lowercase",
            Self::Upper => "UPPERCASE",
            Self::Kebab => "kebab-case",
            Self::Camel => "camelCase",
            Self::Snake => "snake_case",
            Self::Pascal => "PascalCase",
            Self::ScreamingSnake => "SCREAMING_SNAKE_CASE",
        }
    }
}

fn parse_assign_str(input: ParseStream) -> Result<String> {
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => Ok(string.value()),
        other => Err(Error::new(other.span(), "expected string")),
    }
}

fn parse_concrete(input: ParseStream) -> Result<HashMap<syn::Ident, syn::Type>> {
    struct Concrete {
        ident: syn::Ident,
        _equal_token: Token![=],
        ty: syn::Type,
    }

    impl Parse for Concrete {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                _equal_token: input.parse()?,
                ty: input.parse()?,
            })
        }
    }

    let content;
    syn::parenthesized!(content in input);

    Ok(
        Punctuated::<Concrete, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .map(|concrete| (concrete.ident, concrete.ty))
            .collect(),
    )
}

fn parse_assign_inflection(input: ParseStream) -> Result<Inflection> {
    input.parse::<Token![=]>()?;

    match Lit::parse(input)? {
        Lit::Str(string) => Ok(match &*string.value() {
            "lowercase" => Inflection::Lower,
            "UPPERCASE" => Inflection::Upper,
            "camelCase" => Inflection::Camel,
            "snake_case" => Inflection::Snake,
            "PascalCase" => Inflection::Pascal,
            "SCREAMING_SNAKE_CASE" => Inflection::ScreamingSnake,
            "kebab-case" => Inflection::Kebab,
            "SCREAMING-KEBAB-CASE" => Inflection::ScreamingKebab,
            other => {
                syn_err!(
                    string.span();
                    r#"Value "{other}" is not valid for "rename_all". Accepted values are: "lowercase", "UPPERCASE", "camelCase", "snake_case", "PascalCase", "SCREAMING_SNAKE_CASE", "kebab-case" and "SCREAMING-KEBAB-CASE""#
                )
            }
        }),
        other => Err(Error::new(other.span(), "expected string")),
    }
}

fn parse_assign_from_str<T>(input: ParseStream) -> Result<T>
where
    T: Parse,
{
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => string.parse(),
        other => Err(Error::new(other.span(), "expected string")),
    }
}

fn parse_bound(input: ParseStream) -> Result<Vec<WherePredicate>> {
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => {
            let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;

            Ok(string.parse_with(parser)?.into_iter().collect())
        }
        other => Err(Error::new(other.span(), "expected string")),
    }
}
