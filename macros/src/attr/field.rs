use syn::{Attribute, Field, Ident, Result, Type};

use super::{parse_assign_from_str, parse_assign_str, Attr, Serde};
use crate::utils::{parse_attrs, parse_docs};

#[derive(Default)]
pub struct FieldAttr {
    pub type_as: Option<Type>,
    pub type_override: Option<String>,
    pub rename: Option<String>,
    pub inline: bool,
    pub skip: bool,
    pub optional: Optional,
    pub flatten: bool,
    pub docs: String,
}

/// Indicates whether the field is marked with `#[ts(optional)]`.
/// `#[ts(optional)]` turns an `t: Option<T>` into `t?: T`, while
/// `#[ts(optional = nullable)]` turns it into `t?: T | null`.
#[derive(Default)]
pub struct Optional {
    pub optional: bool,
    pub nullable: bool,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = parse_attrs::<Self>(attrs)?;

        #[cfg(feature = "serde-compat")]
        if !result.skip {
            let serde_attr = crate::utils::parse_serde_attrs::<FieldAttr>(attrs);
            result = result.merge(serde_attr.0);
        }

        result.docs = parse_docs(attrs)?;

        Ok(result)
    }
}

impl Attr for FieldAttr {
    type Item = Field;

    fn merge(self, other: Self) -> Self {
        Self {
            type_as: self.type_as.or(other.type_as),
            type_override: self.type_override.or(other.type_override),
            rename: self.rename.or(other.rename),
            inline: self.inline || other.inline,
            skip: self.skip || other.skip,
            optional: Optional {
                optional: self.optional.optional || other.optional.optional,
                nullable: self.optional.nullable || other.optional.nullable,
            },
            flatten: self.flatten || other.flatten,

            // We can't emit TSDoc for a flattened field
            // and we cant make this invalid in assert_validity because
            // this documentation is totally valid in Rust
            docs: if self.flatten || other.flatten {
                String::new()
            } else {
                self.docs + &other.docs
            },
        }
    }

    fn assert_validity(&self, field: &Self::Item) -> Result<()> {
        if self.type_override.is_some() {
            if self.type_as.is_some() {
                syn_err_spanned!(field; "`type` is not compatible with `as`")
            }

            if self.inline {
                syn_err_spanned!(field; "`type` is not compatible with `inline`")
            }

            if self.flatten {
                syn_err_spanned!(
                    field;
                    "`type` is not compatible with `flatten`"
                );
            }
        }

        if self.flatten {
            if self.type_as.is_some() {
                syn_err_spanned!(
                    field;
                    "`as` is not compatible with `flatten`"
                );
            }

            if self.rename.is_some() {
                syn_err_spanned!(
                    field;
                    "`rename` is not compatible with `flatten`"
                );
            }

            if self.inline {
                syn_err_spanned!(
                    field;
                    "`inline` is not compatible with `flatten`"
                );
            }

            if self.optional.optional {
                syn_err_spanned!(
                    field;
                    "`optional` is not compatible with `flatten`"
                );
            }
        }

        if field.ident.is_none() {
            if self.flatten {
                syn_err_spanned!(
                    field;
                    "`flatten` cannot with tuple struct fields"
                );
            }

            if self.rename.is_some() {
                syn_err_spanned!(
                    field;
                    "`flatten` cannot with tuple struct fields"
                );
            }

            if self.optional.optional {
                syn_err_spanned!(
                    field;
                    "`optional` cannot with tuple struct fields"
                );
            }
        }

        Ok(())
    }
}

impl_parse! {
    FieldAttr(input, out) {
        "as" => out.type_as = Some(parse_assign_from_str(input)?),
        "type" => out.type_override = Some(parse_assign_str(input)?),
        "rename" => out.rename = Some(parse_assign_str(input)?),
        "inline" => out.inline = true,
        "skip" => out.skip = true,
        "optional" => {
            use syn::{Token, Error};
            let nullable = if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                let span = input.span();
                match Ident::parse(input)?.to_string().as_str() {
                    "nullable" => true,
                    _ => Err(Error::new(span, "expected 'nullable'"))?
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
    Serde<FieldAttr>(input, out) {
        "rename" => out.0.rename = Some(parse_assign_str(input)?),
        "skip" => out.0.skip = true,
        "flatten" => out.0.flatten = true,
        // parse #[serde(default)] to not emit a warning
        "default" => {
            use syn::Token;
            if input.peek(Token![=]) {
                parse_assign_str(input)?;
            }
        },
    }
}
