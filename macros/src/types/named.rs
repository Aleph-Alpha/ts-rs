use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Field, FieldsNamed, GenericArgument, GenericParam, Generics, Ident, Path, PathArguments,
    PathSegment, Result, Type,
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

    if type_override.is_none() {
        dependencies.push(match inline {
            true => quote! { dependencies.append(&mut <#ty as ts_rs::TS>::dependencies()); },
            false => quote! {
                if <#ty as ts_rs::TS>::transparent() {
                    dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());
                } else {
                    dependencies.push((std::any::TypeId::of::<#ty>(), <#ty as ts_rs::TS>::name()));
                }
            },
        });
    }

    let formatted_ty =
        type_override.map(|t| quote!(#t)).unwrap_or_else(|| {
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
                            && type_path.path == Path::from(PathSegment {
                                ident: type_param.ident.clone(),
                                arguments: PathArguments::None,
                            })
                    )
                })
                .map(|type_param| type_param.ident.to_string())
            {
                quote!(#generic_ident)
            } else if let Some((name, generic_args)) =
                match ty {
                    Type::Path(type_path) => type_path.path.segments.last().and_then(|segment| {
                        match &segment.arguments {
                            PathArguments::AngleBracketed(generic_arguments) => {
                                if has_specialized_impl(&segment.ident) {
                                    None
                                } else {
                                    Some((segment.ident.to_string(), &generic_arguments.args))
                                }
                            }
                            _ => None,
                        }
                    }),
                    _ => None,
                }
            {
                let args = generic_args
                    .iter()
                    .filter_map(|arg| match arg {
                        GenericArgument::Type(Type::Path(type_path)) => {
                            let path = &type_path.path;
                            Some(quote!(<#path as ts_rs::TS>::name()))
                        }
                        _ => None,
                    })
                    .collect::<TokenStream>();
                quote!(format!("{}<{}>", #name, #args))
            } else {
                match inline {
                    true => quote!(<#ty as ts_rs::TS>::inline(indent + 1)),
                    false => quote!(<#ty as ts_rs::TS>::name()),
                }
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

fn has_specialized_impl(ident: &Ident) -> bool {
    ident == "Arc"
        || ident == "Box"
        || ident == "BTreeMap"
        || ident == "BTreeSet"
        || ident == "Cell"
        || ident == "Cow"
        || ident == "HashMap"
        || ident == "HashSet"
        || ident == "Option"
        || ident == "Rc"
        || ident == "RefCell"
        || ident == "Vec"
}
