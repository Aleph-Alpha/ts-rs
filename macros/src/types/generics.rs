use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, GenericParam, Generics, PathArguments, Type};

use crate::deps::Dependencies;

pub fn format_type(ty: &Type, dependencies: &mut Dependencies, generics: &Generics) -> TokenStream {
    // If the type matches one of the generic parameters, just pass the identifier:
    if let Some(generic_ident) = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some(type_param),
            _ => None,
        })
        .find(|type_param| {
            matches!(
                ty,
                Type::Path(type_path)
                    if type_path.qself.is_none()
                    && type_path.path.is_ident(&type_param.ident)
            )
        })
        .map(|type_param| type_param.ident.to_string())
    {
        return quote!(#generic_ident.to_owned());
    }

    dependencies.push_or_append_from(ty);

    match extract_type_args(ty) {
        None => quote!(<#ty as ts_rs::TS>::name()),
        Some(type_args) => {
            let args = type_args
                .iter()
                .map(|ty| format_type(ty, dependencies, generics))
                .collect::<Vec<_>>();
            let args = quote!(vec![#(#args),*]);
            quote!(<#ty as ts_rs::TS>::name_with_type_args(#args))
        }
    }
}

fn extract_type_args(ty: &Type) -> Option<Vec<&Type>> {
    let last_segment = match ty {
        Type::Path(type_path) => type_path.path.segments.last(),
        _ => None,
    }?;

    let segment_arguments = match &last_segment.arguments {
        PathArguments::AngleBracketed(generic_arguments) => Some(generic_arguments),
        _ => None,
    }?;

    let type_args: Vec<_> = segment_arguments
        .args
        .iter()
        .filter_map(|arg| match arg {
            GenericArgument::Type(ty) => Some(ty),
            _ => None,
        })
        .collect();
    if type_args.is_empty() {
        return None;
    }

    Some(type_args)
}
