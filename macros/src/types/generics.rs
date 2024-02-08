use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    GenericArgument, GenericParam, Generics, ItemStruct, PathArguments, Type, TypeGroup,
    TypeReference, TypeSlice, TypeTuple,
};

use crate::{attr::StructAttr, deps::Dependencies};

/// formats the generic arguments (like A, B in struct X<A, B>{..}) as "<X>" where x is a comma
/// seperated list of generic arguments, or an empty string if there are no type generics (lifetime/const generics are ignored).
/// this expands to an expression which evaluates to a `String`.
///
/// If a default type arg is encountered, it will be added to the dependencies.
pub fn format_generics(deps: &mut Dependencies, generics: &Generics) -> TokenStream {
    let mut expanded_params = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some({
                let ty = type_param.ident.to_string();
                if let Some(default) = &type_param.default {
                    let default = format_type(default, deps, generics);
                    quote!(format!("{} = {}", #ty, #default))
                } else {
                    quote!(#ty.to_owned())
                }
            }),
            _ => None,
        })
        .peekable();

    if expanded_params.peek().is_none() {
        return quote!("");
    }

    let comma_separated = quote!([#(#expanded_params),*].join(", "));
    quote!(format!("<{}>", #comma_separated))
}

pub fn format_type(ty: &Type, dependencies: &mut Dependencies, generics: &Generics) -> TokenStream {
    // If the type matches one of the generic parameters, just pass the identifier:
    if let Some(generic) = generics
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
    {
        let generic_ident = generic.ident.clone();
        let generic_ident_str = generic_ident.to_string();

        if !generic.bounds.is_empty() {
            return quote!(#generic_ident_str.to_owned());
        }

        return quote!(
            match <#generic_ident>::inline().as_str() {
                // When exporting a generic, the default type used is `()`,
                // which gives "null" when calling `.name()`. In this case, we
                // want to preserve the type param's identifier as the name used
                "null" => #generic_ident_str.to_owned(),

                // If name is not "null", a type has been provided, so we use its
                // name instead
                x => x.to_owned()
            }
        );
    }

    // special treatment for arrays and tuples
    match ty {
        // Arrays have their own implementation that needs to be handle separetly
        // be cause the T in `[T; N]` is technically not a generic
        Type::Array(type_array) => {
            let formatted = format_type(&type_array.elem, dependencies, generics);
            return quote!(<#type_array>::name_with_type_args(vec![#formatted]));
        }
        // The field is a slice (`[T]`) so it technically doesn't have a
        // generic argument. Therefore, we handle it explicitly here like a `Vec<T>`
        Type::Slice(TypeSlice { ref elem, .. }) => {
            let inner_ty = elem;
            let vec_ty = syn::parse2::<Type>(quote!(Vec::<#inner_ty>)).unwrap();
            return format_type(&vec_ty, dependencies, generics);
        }
        // same goes for a tuple (`(A, B, C)`) - it doesn't have a type arg, so we handle it
        // explicitly here.
        Type::Tuple(tuple) => {
            if tuple.elems.is_empty() {
                // empty tuples `()` should be treated as `null`
                return super::unit::null(&StructAttr::default(), "", &vec![])
                    .unwrap()
                    .inline;
            }

            // we convert the tuple field to a struct: `(A, B, C)` => `struct A(A, B, C)`
            let tuple_struct = super::type_def(
                &StructAttr::default(),
                &vec![],
                &format_ident!("_"),
                &tuple_type_to_tuple_struct(tuple).fields,
                generics,
            )
            .unwrap();
            // now, we return the inline definition
            dependencies.append(tuple_struct.dependencies);
            return tuple_struct.inline;
        }
        Type::Reference(syn::TypeReference { ref elem, .. }) => {
            return format_type(elem, dependencies, generics)
        }
        _ => (),
    };

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
        Type::Group(TypeGroup { elem, .. }) | Type::Reference(TypeReference { elem, .. }) => {
            return extract_type_args(elem)
        }
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

// convert a [`TypeTuple`],  e.g `(A, B, C)`
//      to a [`ItemStruct`], e.g `struct A(A, B, C)`
fn tuple_type_to_tuple_struct(tuple: &TypeTuple) -> ItemStruct {
    let elements = tuple.elems.iter();
    syn::parse2(quote!(struct A( #(#elements),* );))
        .expect("could not convert tuple to tuple struct")
}
