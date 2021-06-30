use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Field, FieldsNamed, GenericArgument, GenericParam, Generics, PathArguments, Result, Type,
};

use crate::attr::{FieldAttr, Inflection};
use crate::DerivedTS;

pub(crate) fn named(
    name: &str,
    rename_all: &Option<Inflection>,
    fields: &FieldsNamed,
    generics: &Generics,
) -> Result<DerivedTS> {
    let mut formatted_fields = vec![];
    let mut dependencies = vec![];
    for field in &fields.named {
        format_field(
            &mut formatted_fields,
            &mut dependencies,
            field,
            &rename_all,
            generics,
        )?;
    }

    let fields = quote!(vec![#(#formatted_fields),*].join("\n"));
    let generic_args = match &generics.params {
        params if !params.is_empty() => {
            let expanded_params = params
                .iter()
                .filter_map(|param| match param {
                    GenericParam::Type(type_param) => Some(type_param.ident.to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(", ");
            quote!(format!("<{}>", #expanded_params))
        }
        _ => quote!("".to_owned()),
    };

    Ok(DerivedTS {
        inline: quote! {
            format!(
                "{{\n{}\n{}}}",
                #fields,
                " ".repeat(indent * 4)
            )
        },
        decl: quote!(format!("interface {}{} {}", #name, #generic_args, Self::inline(0))),
        inline_flattened: Some(fields),
        name: name.to_owned(),
        dependencies: quote! {
            let mut dependencies = vec![];
            #( #dependencies )*
            dependencies
        },
    })
}

// build an expresion which expands to a string, representing a single field of a struct.
fn format_field(
    formatted_fields: &mut Vec<TokenStream>,
    dependencies: &mut Vec<TokenStream>,
    field: &Field,
    rename_all: &Option<Inflection>,
    generics: &Generics,
) -> Result<()> {
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip,
        flatten,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(());
    }

    let ty = &field.ty;

    if flatten {
        match (&type_override, &rename, inline) {
            (Some(_), _, _) => syn_err!("`type` is not compatible with `flatten`"),
            (_, Some(_), _) => syn_err!("`rename` is not compatible with `flatten`"),
            (_, _, true) => syn_err!("`inline` is not compatible with `flatten`"),
            _ => {}
        }

        formatted_fields.push(quote!(<#ty as ts_rs::TS>::inline_flattened(indent)));
        dependencies.push(quote!(dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());));
        return Ok(());
    }

    let formatted_ty = type_override.map(|t| quote!(#t)).unwrap_or_else(|| {
        if inline {
            dependencies
                .push(quote!(dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());));
            quote!(<#ty as ts_rs::TS>::inline(indent + 1))
        } else {
            format_type(ty, dependencies, generics)
        }
    });
    let name = match (rename, rename_all) {
        (Some(rn), _) => rn,
        (None, Some(rn)) => rn.apply(&field.ident.as_ref().unwrap().to_string()),
        (None, None) => field.ident.as_ref().unwrap().to_string(),
    };

    formatted_fields.push(quote! {
        format!("{}{}: {},", " ".repeat((indent + 1) * 4), #name, #formatted_ty)
    });

    Ok(())
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

fn format_type(ty: &Type, dependencies: &mut Vec<TokenStream>, generics: &Generics) -> TokenStream {
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

    dependencies.push(quote! {
        if <#ty as ts_rs::TS>::transparent() {
            dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());
        } else {
            dependencies.push((std::any::TypeId::of::<#ty>(), <#ty as ts_rs::TS>::name()));
        }
    });

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
