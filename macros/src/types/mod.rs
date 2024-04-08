use syn::{
    parse_quote, AngleBracketedGenericArguments, AssocType, Fields, GenericArgument, Ident,
    ItemStruct, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, Result,
    ReturnType, Type, TypeArray, TypeGroup, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice,
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

pub(super) fn type_as_infer(type_as: &Type, original_type: &Type) -> Type {
    use PathArguments as P;

    let recurse = |ty: &Type| -> Type { type_as_infer(ty, original_type) };

    match type_as {
        Type::Infer(_) => original_type.clone(),
        Type::Array(TypeArray { elem, len, .. }) => {
            let elem = recurse(elem);
            parse_quote!([#elem; #len])
        }
        Type::Group(TypeGroup { elem, group_token }) => Type::Group(TypeGroup {
            group_token: *group_token,
            elem: Box::new(recurse(elem)),
        }),
        Type::Paren(TypeParen { elem, .. }) => {
            let elem = recurse(elem);
            parse_quote![(#elem)]
        }
        Type::Path(TypePath { path, qself: None }) => Type::Path(TypePath {
            path: Path {
                leading_colon: path.leading_colon,
                segments: path
                    .segments
                    .iter()
                    .map(|x| match x.arguments {
                        P::None => x.clone(),
                        P::Parenthesized(ParenthesizedGenericArguments {
                            ref inputs,
                            ref output,
                            paren_token,
                        }) => PathSegment {
                            ident: x.ident.clone(),
                            arguments: PathArguments::Parenthesized(
                                ParenthesizedGenericArguments {
                                    paren_token,
                                    inputs: inputs.iter().map(recurse).collect(),
                                    output: match output {
                                        ReturnType::Default => ReturnType::Default,
                                        ReturnType::Type(r_arrow, ty) => {
                                            ReturnType::Type(*r_arrow, Box::new(recurse(ty)))
                                        }
                                    },
                                },
                            ),
                        },
                        P::AngleBracketed(ref angle_bracketed) => PathSegment {
                            ident: x.ident.clone(),
                            arguments: P::AngleBracketed(type_as_infer_angle_bracketed(
                                angle_bracketed,
                                original_type,
                            )),
                        },
                    })
                    .collect(),
            },
            qself: None,
        }),
        Type::Ptr(TypePtr {
            elem,
            star_token,
            const_token,
            mutability,
        }) => {
            let elem = recurse(elem);
            parse_quote!(#star_token #const_token #mutability #elem)
        }
        Type::Reference(TypeReference {
            elem,
            lifetime,
            and_token,
            mutability,
        }) => {
            let elem = recurse(elem);
            parse_quote!(#and_token #lifetime #mutability #elem)
        }
        Type::Slice(TypeSlice { elem, .. }) => {
            let elem = recurse(elem);
            parse_quote!([#elem])
        }
        Type::Tuple(TypeTuple { elems, .. }) => {
            let elems = elems.iter().map(recurse).collect::<Vec<_>>();
            parse_quote![(#(#elems),*)]
        }
        x => x.clone(),
    }
}

fn type_as_infer_angle_bracketed(
    angle_bracketed: &AngleBracketedGenericArguments,
    original_type: &Type,
) -> AngleBracketedGenericArguments {
    let recurse = |args: &AngleBracketedGenericArguments| -> AngleBracketedGenericArguments {
        type_as_infer_angle_bracketed(args, original_type)
    };

    AngleBracketedGenericArguments {
        colon2_token: angle_bracketed.colon2_token,
        gt_token: angle_bracketed.gt_token,
        lt_token: angle_bracketed.lt_token,
        args: angle_bracketed
            .args
            .iter()
            .map(|arg| match arg {
                GenericArgument::Type(ty) => {
                    GenericArgument::Type(type_as_infer(ty, original_type))
                }
                GenericArgument::AssocType(assoc_ty) => GenericArgument::AssocType(AssocType {
                    ident: assoc_ty.ident.clone(),
                    generics: assoc_ty.generics.as_ref().map(recurse),
                    eq_token: assoc_ty.eq_token,
                    ty: type_as_infer(&assoc_ty.ty, original_type),
                }),
                _ => arg.clone(),
            })
            .collect(),
    }
}
