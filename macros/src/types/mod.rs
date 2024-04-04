use quote::{quote, ToTokens};
use syn::{
    Fields, Ident, ItemStruct, Result, Type, TypeArray, TypeParen, TypeReference, TypeSlice,
    TypeTuple,
};

use crate::{
    attr::{Attr, StructAttr},
    utils::to_ts_ident,
    DerivedTS,
};

mod r#enum;
mod named;
mod newtype;
mod tuple;
mod type_as;
mod type_override;
mod unit;

pub(crate) use r#enum::r#enum_def;

pub(crate) fn struct_def(s: &ItemStruct) -> Result<DerivedTS> {
    let attr = StructAttr::from_attrs(&s.attrs)?;

    type_def(&attr, &s.ident, &s.fields)
}

fn type_def(attr: &StructAttr, ident: &Ident, fields: &Fields) -> Result<DerivedTS> {
    attr.assert_validity(fields)?;

    let name = attr.rename.clone().unwrap_or_else(|| to_ts_ident(ident));
    if let Some(attr_type_override) = &attr.type_override {
        return type_override::type_override_struct(attr, &name, attr_type_override);
    }
    if let Some(attr_type_as) = &attr.type_as {
        return type_as::type_as_struct(attr, &name, attr_type_as);
    }

    match fields {
        Fields::Named(named) => match named.named.len() {
            0 => unit::empty_object(attr, &name),
            _ => named::named(attr, &name, named),
        },
        Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => unit::empty_array(attr, &name),
            1 => newtype::newtype(attr, &name, unnamed),
            _ => tuple::tuple(attr, &name, unnamed),
        },
        Fields::Unit => unit::null(attr, &name),
    }
}

#[allow(unused)]
pub(super) fn type_as_infer(type_as: &Type, original_type: &Type) -> Result<Type> {
    syn::parse2(
        type_as
            .to_token_stream()
            .into_iter()
            .map(|x| {
                let ty = syn::parse2::<Type>(x.to_token_stream());
                Ok(match ty {
                    Ok(Type::Infer(_)) => original_type.to_token_stream(),
                    Ok(Type::Reference(TypeReference {
                        elem,
                        lifetime,
                        and_token,
                        mutability,
                    })) => {
                        let elem = type_as_infer(&elem, original_type)?;
                        quote!(#and_token #lifetime #mutability #elem)
                    }
                    Ok(Type::Array(TypeArray { elem, len, .. })) => {
                        let elem = type_as_infer(&elem, original_type)?;
                        quote!([#elem; #len])
                    }
                    Ok(Type::Tuple(TypeTuple { elems, .. })) => {
                        let elems = elems
                            .iter()
                            .map(|x| type_as_infer(x, original_type))
                            .collect::<Result<Vec<_>>>()?;
                        quote![(#(#elems),*)]
                    }
                    Ok(Type::Slice(TypeSlice { elem, .. })) => {
                        let elem = type_as_infer(&elem, original_type)?;
                        quote!([#elem])
                    }
                    Ok(Type::Paren(TypeParen { elem, .. })) => {
                        let elem = type_as_infer(&elem, original_type)?;
                        quote![(elem)]
                    }
                    y => x.to_token_stream(),
                })
            })
            .collect::<Result<proc_macro2::TokenStream>>()?,
    )
}
