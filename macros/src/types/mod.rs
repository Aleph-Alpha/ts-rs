use syn::{
    AngleBracketedGenericArguments, Fields, GenericArgument, Ident, ItemStruct, PathArguments,
    Result, ReturnType, Type, TypeArray, TypeGroup, TypeParen, TypePath, TypePtr, TypeReference,
    TypeSlice, TypeTuple,
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

pub(super) fn replace_underscore(ty: &mut Type, with: &Type) {
    match ty {
        Type::Infer(_) => *ty = with.clone(),
        Type::Array(TypeArray { elem, .. })
        | Type::Group(TypeGroup { elem, .. })
        | Type::Paren(TypeParen { elem, .. })
        | Type::Ptr(TypePtr { elem, .. })
        | Type::Reference(TypeReference { elem, .. })
        | Type::Slice(TypeSlice { elem, .. }) => {
            replace_underscore(elem, with);
        }
        Type::Tuple(TypeTuple { elems, .. }) => {
            for elem in elems {
                replace_underscore(elem, with);
            }
        }
        Type::Path(TypePath { path, qself: None }) => {
            for segment in &mut path.segments {
                match &mut segment.arguments {
                    PathArguments::None => (),
                    PathArguments::AngleBracketed(a) => {
                        replace_underscore_in_angle_bracketed(a, with);
                    }
                    PathArguments::Parenthesized(p) => {
                        for input in &mut p.inputs {
                            replace_underscore(input, with);
                        }
                        if let ReturnType::Type(_, output) = &mut p.output {
                            replace_underscore(output, with);
                        }
                    }
                }
            }
        }
        _ => (),
    }
}

fn replace_underscore_in_angle_bracketed(args: &mut AngleBracketedGenericArguments, with: &Type) {
    for arg in &mut args.args {
        match arg {
            GenericArgument::Type(ty) => {
                replace_underscore(ty, with);
            }
            GenericArgument::AssocType(assoc_ty) => {
                replace_underscore(&mut assoc_ty.ty, with);
                for g in &mut assoc_ty.generics {
                    replace_underscore_in_angle_bracketed(g, with);
                }
            }
            _ => (),
        }
    }
}
