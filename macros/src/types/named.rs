use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Field, FieldsNamed, GenericArgument, PathArguments, Result, Type, parse_quote, Path};

use crate::{
    attr::{FieldAttr, Inflection, Optional, StructAttr},
    deps::Dependencies,
    utils::{raw_name_to_ts_field, to_ts_ident},
    DerivedTS,
};

pub(crate) fn named(attr: &StructAttr, name: &str, fields: &FieldsNamed) -> Result<DerivedTS> {
    let crate_rename = attr
        .crate_rename
        .clone()
        .unwrap_or_else(|| parse_quote!(::ts_rs));

    let mut formatted_fields = Vec::new();
    let mut flattened_fields = Vec::new();
    let mut dependencies = Dependencies::new(crate_rename.clone());

    if let Some(tag) = &attr.tag {
        let formatted = format!("{}: \"{}\",", tag, name);
        formatted_fields.push(quote! {
            #formatted.to_string()
        });
    }

    for field in &fields.named {
        format_field(
            &crate_rename,
            &mut formatted_fields,
            &mut flattened_fields,
            &mut dependencies,
            field,
            &attr.rename_all,
        )?;
    }

    let fields = quote!(<[String]>::join(&[#(#formatted_fields),*], " "));
    let flattened = quote!(<[String]>::join(&[#(#flattened_fields),*], " & "));

    let inline = match (formatted_fields.len(), flattened_fields.len()) {
        (0, 0) => quote!("{  }".to_owned()),
        (_, 0) => quote!(format!("{{ {} }}", #fields)),
        (0, 1) => quote!(#flattened.trim_matches(|c| c == '(' || c == ')').to_owned()),
        (0, _) => quote!(#flattened),
        (_, _) => quote!(format!("{{ {} }} & {}", #fields, #flattened)),
    };

    Ok(DerivedTS {
        crate_rename,
        // the `replace` combines `{ ... } & { ... }` into just one `{ ... }`. Not necessary, but it
        // results in simpler type definitions.
        inline: quote!(#inline.replace(" } & { ", " ")),
        inline_flattened: Some(quote!(format!("{{ {} }}", #fields))),
        docs: attr.docs.clone(),
        dependencies,
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

// build an expression which expands to a string, representing a single field of a struct.
//
// formatted_fields will contain all the fields that do not contain the flatten
// attribute, in the format
// key: type,
//
// flattened_fields will contain all the fields that contain the flatten attribute
// in their respective formats, which for a named struct is the same as formatted_fields,
// but for enums is
// ({ /* variant data */ } | { /* variant data */ })
fn format_field(
    crate_rename: &Path,
    formatted_fields: &mut Vec<TokenStream>,
    flattened_fields: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    field: &Field,
    rename_all: &Option<Inflection>,
) -> Result<()> {
    let FieldAttr {
        type_as,
        type_override,
        rename,
        inline,
        skip,
        optional,
        flatten,
        docs,
    } = FieldAttr::from_attrs(&field.attrs)?;

    if skip {
        return Ok(());
    }

    if type_as.is_some() && type_override.is_some() {
        syn_err_spanned!(field; "`type` is not compatible with `as`")
    }

    let parsed_ty = type_as.as_ref().unwrap_or(&field.ty).clone();

    let (ty, optional_annotation) = match optional {
        Optional {
            optional: true,
            nullable,
        } => {
            let inner_type = extract_option_argument(&parsed_ty)?; // inner type of the optional
            match nullable {
                true => (&parsed_ty, "?"),  // if it's nullable, we keep the original type
                false => (inner_type, "?"), // if not, we use the Option's inner type
            }
        }
        Optional {
            optional: false, ..
        } => (&parsed_ty, ""),
    };

    if flatten {
        match (&type_as, &type_override, &rename, inline) {
            (Some(_), _, _, _) => syn_err_spanned!(field; "`as` is not compatible with `flatten`"),
            (_, Some(_), _, _) => {
                syn_err_spanned!(field; "`type` is not compatible with `flatten`")
            }
            (_, _, Some(_), _) => {
                syn_err_spanned!(field; "`rename` is not compatible with `flatten`")
            }
            (_, _, _, true) => syn_err_spanned!(field; "`inline` is not compatible with `flatten`"),
            _ => {}
        }

        flattened_fields.push(quote!(<#ty as #crate_rename::TS>::inline_flattened()));
        dependencies.append_from(ty);
        return Ok(());
    }

    let formatted_ty = type_override.map(|t| quote!(#t)).unwrap_or_else(|| {
        if inline {
            dependencies.append_from(ty);
            quote!(<#ty as #crate_rename::TS>::inline())
        } else {
            dependencies.push(ty);
            quote!(<#ty as #crate_rename::TS>::name())
        }
    });
    let field_name = to_ts_ident(field.ident.as_ref().unwrap());
    let name = match (rename, rename_all) {
        (Some(rn), _) => rn,
        (None, Some(rn)) => rn.apply(&field_name),
        (None, None) => field_name,
    };
    let valid_name = raw_name_to_ts_field(name);

    // Start every doc string with a newline, because when other characters are in front, it is not "understood" by VSCode
    let docs = match docs.is_empty() {
        true => "".to_string(),
        false => format!("\n{}", &docs),
    };

    formatted_fields.push(quote! {
        format!("{}{}{}: {},", #docs, #valid_name, #optional_annotation, #formatted_ty)
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
                        other => syn_err!(other.span(); "`Option` argument must be a type"),
                    }
                }
                other => {
                    syn_err!(other.span(); "`Option` type must have a single generic argument")
                }
            }
        }
        other => syn_err!(other.span(); "`optional` can only be used on an Option<T> type"),
    }
}
