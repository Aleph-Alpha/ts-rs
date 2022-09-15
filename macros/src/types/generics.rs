use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, GenericParam, Generics, ItemStruct, PathArguments, Type, TypeTuple};

use crate::{attr::StructAttr, deps::Dependencies};

/// formats the generic arguments (like A, B in struct X<A, B>{..}) as "<X>" where x is a comma
/// seperated list of generic arguments, or an empty string if there are no generics.
/// this expands to an expression which evaluates to a `String`.
///
/// If a default type arg is encountered, it will be added to the dependencies.
pub fn format_generics(deps: &mut Dependencies, generics: &Generics) -> TokenStream {
    if generics.params.is_empty() {
        return quote!("");
    }

    let expanded_params = generics.params.iter().filter_map(|param| match param {
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
    });

    let comma_separated = quote!({
        let v: Vec<String> = vec![#(#expanded_params),*];
        v.join(", ")
    });
    quote!(format!("<{}>", #comma_separated))
}

pub fn format_type(ty: &Type, dependencies: &mut Dependencies, generics: &Generics) -> TokenStream {
    // Remap all lifetimes to 'static in ty.
    struct Visitor;
    impl syn::visit_mut::VisitMut for Visitor {
        fn visit_type_mut(&mut self, ty: &mut Type) {
            match ty {
                Type::Reference(ref_type) => {
                    ref_type.lifetime = ref_type
                        .lifetime
                        .as_ref()
                        .map(|_| syn::parse2(quote!('static)).unwrap());
                }
                _ => {}
            }
            syn::visit_mut::visit_type_mut(self, ty);
        }

        fn visit_generic_argument_mut(&mut self, ga: &mut GenericArgument) {
            match ga {
                GenericArgument::Lifetime(lt) => {
                    *lt = syn::parse2(quote!('static)).unwrap();
                }
                _ => {}
            }
            syn::visit_mut::visit_generic_argument_mut(self, ga);
        }
    }
    use syn::visit_mut::VisitMut;
    let mut ty = ty.clone();
    Visitor.visit_type_mut(&mut ty);

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
                &ty,
                Type::Path(type_path)
                    if type_path.qself.is_none()
                    && type_path.path.is_ident(&type_param.ident)
            )
        })
        .map(|type_param| type_param.ident.to_string())
    {
        return quote!(#generic_ident.to_owned());
    }

    // special treatment for arrays and tuples
    match ty {
        // the field is an array (`[T; n]`) so it technically doesn't have a generic argument.
        // therefore, we handle it explicitly here like a `Vec<T>`
        Type::Array(array) => {
            let inner_ty = &array.elem;
            let vec_ty = syn::parse2::<Type>(quote!(Vec::<#inner_ty>)).unwrap();
            return format_type(&vec_ty, dependencies, generics);
        }
        // same goes for a tuple (`(A, B, C)`) - it doesn't have a type arg, so we handle it
        // explicitly here.
        Type::Tuple(tuple) => {
            // we convert the tuple field to a struct: `(A, B, C)` => `struct A(A, B, C)`
            let tuple_struct = super::type_def(
                &StructAttr::default(),
                &format_ident!("_"),
                &tuple_type_to_tuple_struct(&tuple).fields,
                generics,
            )
            .unwrap();
            // now, we return the inline definition
            dependencies.append(tuple_struct.dependencies);
            return tuple_struct.inline;
        }
        _ => (),
    };

    dependencies.push_or_append_from(&ty);
    match extract_type_args(&ty) {
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

// convert a [`TypeTuple`],  e.g `(A, B, C)`
//      to a [`ItemStruct`], e.g `struct A(A, B, C)`
fn tuple_type_to_tuple_struct(tuple: &TypeTuple) -> ItemStruct {
    let elements = tuple.elems.iter();
    syn::parse2(quote!(struct A( #(#elements),* );))
        .expect("could not convert tuple to tuple struct")
}
