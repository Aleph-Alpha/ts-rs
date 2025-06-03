use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, parse_quote_spanned, Error, Expr, Ident, Path, Token, Type,
};

use crate::attr::FieldAttr;

/// Indicates whether the field is marked with `#[ts(optional)]`.
/// `#[ts(optional)]` turns an `t: Option<T>` into `t?: T`, while
/// `#[ts(optional = nullable)]` turns it into `t?: T | null`.
#[derive(Default, Clone, Copy)]
pub enum Optional {
    Optional {
        nullable: bool,
    },

    #[default]
    NotOptional,
}

impl Optional {
    pub fn or(self, other: Optional) -> Self {
        match (self, other) {
            (Self::NotOptional, Self::NotOptional) => Self::NotOptional,

            (Self::Optional { nullable }, Self::NotOptional)
            | (Self::NotOptional, Self::Optional { nullable }) => Self::Optional { nullable },

            (Self::Optional { nullable: a }, Self::Optional { nullable: b }) => {
                Self::Optional { nullable: a || b }
            }
        }
    }
}

pub fn parse_optional(input: ParseStream) -> syn::Result<Optional> {
    let nullable = if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let span = input.span();
        match Ident::parse(input)?.to_string().as_str() {
            "nullable" => true,
            _ => Err(Error::new(span, "expected 'nullable'"))?,
        }
    } else {
        false
    };

    Ok(Optional::Optional { nullable })
}

/// Given a field, return a tuple `(is_optional, type)`.  
///
/// `is_optional`:  
/// An expression evaluating to bool, indicating whether the field should be annotated with `?`.
///
/// `type`:  
/// The transformed type of the field after applying the `#[ts(optional)]` annotation.
/// For a field of type `Option<T>`, the returned type is `T`. For all other types `T`, it's `T`.
pub fn apply(
    crate_rename: &Path,
    for_struct: Optional,
    field_ty: &Type,
    field_attr: &FieldAttr,
    span: Span,
) -> (Expr, Type) {
    let (is_optional, nullable) = match (
        for_struct,
        field_attr.optional,
        field_attr.maybe_omitted && field_attr.has_default,
    ) {
        // `#[ts(optional)]` on field takes precedence, and is enforced **AT COMPILE TIME**
        (_, Optional::Optional { nullable }, _) => (
            // expression that evaluates to the string "?", but fails to compile if `ty` is not an `Option`.
            parse_quote_spanned! { span => {
                fn check_that_field_is_option<T: #crate_rename::IsOption>(_: std::marker::PhantomData<T>) {}
                let x: std::marker::PhantomData<#field_ty> = std::marker::PhantomData;
                check_that_field_is_option(x);
                true
            }},
            nullable,
        ),
        // `#[ts(optional)]` on the struct acts as `#[ts(optional)]` on a field, but does not error on non-`Option`
        // fields. Instead, it is a no-op.
        (Optional::Optional { nullable }, _, _) => (
            parse_quote! {
                <#field_ty as #crate_rename::TS>::IS_OPTION
            },
            nullable,
        ),
        // field may be omitted during serialization and has a default value, so the field can be
        // optional.
        (_, _, true) => (parse_quote!(true), true),
        _ => (parse_quote!(false), true),
    };

    let ty = if nullable {
        field_ty.clone()
    } else {
        parse_quote! {<#field_ty as #crate_rename::TS>::OptionInnerType}
    };

    (is_optional, ty)
}
