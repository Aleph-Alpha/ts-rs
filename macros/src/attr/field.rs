use syn::{spanned::Spanned, Attribute, Ident, Result};

use super::parse_assign_str;
use crate::utils::{parse_attrs, parse_docs};

#[derive(Default)]
pub struct FieldAttr {
    pub type_as: Option<String>,
    pub type_override: Option<String>,
    pub rename: Option<String>,
    pub inline: bool,
    pub skip: bool,
    pub optional: Optional,
    pub flatten: bool,
    pub docs: Vec<String>,
}

/// Indicates whether the field is marked with `#[ts(optional)]`.
/// `#[ts(optional)]` turns an `t: Option<T>` into `t?: T`, while
/// `#[ts(optional = nullable)]` turns it into `t?: T | null`.
#[derive(Default)]
pub struct Optional {
    pub optional: bool,
    pub nullable: bool,
}

#[cfg(feature = "serde-compat")]
#[derive(Default)]
pub struct SerdeFieldAttr(FieldAttr);

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        result.docs = parse_docs(attrs)?;
        #[cfg(feature = "serde-compat")]
        if !result.skip {
            crate::utils::parse_serde_attrs::<SerdeFieldAttr>(attrs)
                .for_each(|a| result.merge(a.0));
        }
        Ok(result)
    }

    fn merge(
        &mut self,
        FieldAttr {
            type_as,
            type_override,
            rename,
            inline,
            skip,
            optional: Optional { optional, nullable },
            flatten,
            mut docs,
        }: FieldAttr,
    ) {
        self.rename = self.rename.take().or(rename);
        self.type_as = self.type_as.take().or(type_as);
        self.type_override = self.type_override.take().or(type_override);
        self.inline = self.inline || inline;
        self.skip = self.skip || skip;
        self.optional = Optional {
            optional: self.optional.optional || optional,
            nullable: self.optional.nullable || nullable,
        };
        self.flatten |= flatten;
        self.docs.append(&mut docs);
    }
}

impl_parse! {
    FieldAttr(input, out) {
        "as" => out.type_as = Some(parse_assign_str(input)?),
        "type" => out.type_override = Some(parse_assign_str(input)?),
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "inline" => out.inline = true,
        "skip" => out.skip = true,
        "optional" => {
          use syn::{Token, Error};
            let nullable = if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                match Ident::parse(input)?.to_string().as_str() {
                    "nullable" => true,
                    other => Err(Error::new(other.span(), "expected 'nullable'"))?
                }
            } else {
                false
            };
            out.optional = Optional {
                optional: true,
                nullable,
            }
        },
        "flatten" => out.flatten = true,
    }
}

#[cfg(feature = "serde-compat")]
impl_parse! {
    SerdeFieldAttr(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "skip" => out.0.skip = true,
        "flatten" => out.0.flatten = true,
        // parse #[serde(default)] to not emit a warning
        "default" => {
            use syn::Token;
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                parse_assign_str(input)?;
            }
        },
    }
}
