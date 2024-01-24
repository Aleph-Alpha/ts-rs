use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, FieldsNamed, GenericArgument, Generics, PathArguments, Result, Type};

use crate::{
    attr::{FieldAttr, Inflection, StructAttr},
    deps::Dependencies,
    types::generics::{format_generics, format_type},
    utils::{raw_name_to_ts_field, to_ts_ident},
    DerivedTS,
};

pub(crate) fn named(
    attr: &StructAttr,
    name: &str,
    fields: &FieldsNamed,
    generics: &Generics,
) -> Result<DerivedTS> {
    let mut formatted_fields = Vec::new();
    let mut dependencies = Dependencies::default();
    if let Some(tag) = &attr.tag {
        let formatted = format!("{}: \"{}\",", tag, name);
        formatted_fields.push(quote! {
            #formatted.to_string()
        });
    }

    for field in &fields.named {
        format_field(
            &mut formatted_fields,
            &mut dependencies,
            field,
            &attr.rename_all,
            generics,
        )?;
    }

    let fields = quote! {
        {
            let fields: Vec<String> = vec![#(#formatted_fields),*];
            let enums = fields
                .iter()
                .filter(|x| {
                    x.contains('{') && match x.find('|') {
                        Some(i) => &x[i..] != "| null,", // Avoid mistaking `Option` by flattened enum
                        None => false,
                    }
                })
                .cloned()
                .collect::<Box<[_]>>();
            let other_fields = fields
                .iter()
                .filter(|x| !x.contains('{') || match x.find('|') {
                    Some(i) => &x[i..] == "| null,",
                    None => true,
                })
                .cloned()
                .collect::<Box<[_]>>();

            match (other_fields.len(), enums.len()) {
                (0, 0) => "{ }".to_owned(), // This is the best representation of an empty TS object
                (_, 0) => format!(
                    "{{ {} }}",
                    other_fields.join(" ")
                ),
                (0, 1) => {
                    enums[0].to_owned()
                },
                (0, _) => {
                    enums.iter().map(|x| format!("({})", x)).collect::<Box<[_]>>().join(" & ")
                },
                (_, _) => {
                    format!(
                        "{{ {} }} & {}",
                        other_fields.join(" "),
                        enums.iter().map(|x| format!("({})", x)).collect::<Box<[_]>>().join(" & ")
                    )
                }
            }
        }
    };
    let generic_args = format_generics(&mut dependencies, generics);

    Ok(DerivedTS {
        inline: fields.clone(),
        decl: quote!(format!("type {}{} = {}", #name, #generic_args, Self::inline())),
        inline_flattened: Some(
            quote!({
                println!("{}", &#fields);
                #fields.trim_start_matches("{ ").trim_end_matches(" }").to_owned()
            }),
        ),
        name: name.to_owned(),
        dependencies,
        export: attr.export,
        export_to: attr.export_to.clone(),
    })
}

// build an expresion which expands to a string, representing a single field of a struct.
fn format_field(
    formatted_fields: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    field: &Field,
    rename_all: &Option<Inflection>,
    generics: &Generics,
) -> Result<()> {
    let FieldAttr {
        type_override,
        rename,
        inline,
        skip,
        optional,
        flatten,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(());
    }

    let (ty, optional_annotation) = match optional {
        true => (extract_option_argument(&field.ty)?, "?"),
        false => (&field.ty, ""),
    };

    if flatten {
        match (&type_override, &rename, inline) {
            (Some(_), _, _) => syn_err!("`type` is not compatible with `flatten`"),
            (_, Some(_), _) => syn_err!("`rename` is not compatible with `flatten`"),
            (_, _, true) => syn_err!("`inline` is not compatible with `flatten`"),
            _ => {}
        }

        formatted_fields.push(quote!(<#ty as ts_rs::TS>::inline_flattened()));
        dependencies.append_from(ty);
        return Ok(());
    }

    let formatted_ty = type_override.map(|t| quote!(#t)).unwrap_or_else(|| {
        if inline {
            dependencies.append_from(ty);
            quote!(<#ty as ts_rs::TS>::inline())
        } else {
            format_type(ty, dependencies, generics)
        }
    });
    let field_name = to_ts_ident(field.ident.as_ref().unwrap());
    let name = match (rename, rename_all) {
        (Some(rn), _) => rn,
        (None, Some(rn)) => rn.apply(&field_name),
        (None, None) => field_name,
    };
    let valid_name = raw_name_to_ts_field(name);

    formatted_fields.push(quote! {
        format!("{}{}: {},", #valid_name, #optional_annotation, #formatted_ty)
    });

    Ok(())
}

fn extract_option_argument(ty: &Type) -> Result<&Type> {
    match ty {
        Type::Path(type_path)
            if type_path.qself.is_none()
                && type_path.path.leading_colon.is_none()
                && type_path.path.segments.len() == 1
                && type_path.path.segments[0].ident == "Option" =>
        {
            let segment = &type_path.path.segments[0];
            match &segment.arguments {
                PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                    match &args.args[0] {
                        GenericArgument::Type(inner_ty) => Ok(inner_ty),
                        _ => syn_err!("`Option` argument must be a type"),
                    }
                }
                _ => syn_err!("`Option` type must have a single generic argument"),
            }
        }
        _ => syn_err!("`optional` can only be used on an Option<T> type"),
    }
}
