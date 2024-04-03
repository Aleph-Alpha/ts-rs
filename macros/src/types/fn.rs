use std::{collections::HashMap, ops::Not};

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, Field, FnArg,
    ItemFn, PatType, Result, TypeReference,
};

use crate::{
    attr::{Args, FnAttr},
    deps::Dependencies,
    utils::{parse_docs, to_ts_ident},
    DerivedTS,
};

pub struct ParsedFn {
    pub args_struct: Option<TokenStream>,
    pub derived_fn: DerivedTS,
}

pub fn fn_def(input: &ItemFn, fn_attr: FnAttr) -> Result<ParsedFn> {
    let mut dependencies = Dependencies::new(fn_attr.crate_rename());

    let ident = &input.sig.ident;
    let generics = &input.sig.generics;
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let struct_ident = format_ident!("{}Args", ident.to_string().to_pascal_case());
    let fields = input
        .sig
        .inputs
        .iter()
        .map(|x| match x {
            FnArg::Receiver(_) => Err(Error::new(x.span(), "self parameter is not allowed")),
            FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                dependencies.push(ty);
                Ok(Field {
                    attrs: attrs.to_vec(),
                    vis: syn::Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: Some(syn::parse2(pat.to_token_stream())?),
                    colon_token: None,
                    ty: match ty.as_ref() {
                        syn::Type::Reference(TypeReference { elem, .. }) => {
                            parse_quote!(Box<#elem>)
                        }
                        x => x.clone(),
                    },
                })
            }
        })
        .collect::<Result<Punctuated<_, Comma>>>()?;

    let crate_rename = fn_attr.crate_rename();
    let FnAttr {
        rename_all,
        rename,
        args,
        export_to,
        ..
    } = fn_attr;
    let struct_attr = rename_all.map(|rename_all| {
        let rename_all = rename_all.as_str();
        Some(quote!(#[ts(rename_all = #rename_all)]))
    });

    let args_struct = fields.is_empty().not().then_some(quote!(
        #[derive(#crate_rename::TS)]
        #struct_attr
        struct #struct_ident #ty_generics #where_clause {
            #fields
        }
    ));

    let docs = parse_docs(&input.attrs)?;
    let ts_name = rename
        .clone()
        .unwrap_or_else(|| to_ts_ident(ident))
        .to_pascal_case();
    let is_async = input.sig.asyncness.is_some();
    let return_ty = match (is_async, input.sig.output.clone()) {
        (false, syn::ReturnType::Default) => quote!("void"),
        (true, syn::ReturnType::Default) => quote!("Promise<void>"),
        (false, syn::ReturnType::Type(_, ref ty)) => {
            dependencies.push(ty);
            quote!(<#ty as #crate_rename::TS>::name())
        }
        (true, syn::ReturnType::Type(_, ref ty)) => {
            dependencies.push(ty);
            quote!(format!("Promise<{}>", <#ty as #crate_rename::TS>::name()))
        }
    };

    let inline = match (&args_struct, args) {
        (Some(_), Args::Inlined) => quote!(format!(
            "(args: {}) => {}",
            <#struct_ident as #crate_rename::TS>::inline(),
            #return_ty,
        )),
        (Some(_), Args::Flattened) => quote!(format!("({}) => {}",
            <#struct_ident as #crate_rename::TS>::inline_flattened().trim_matches(['{', '}', ' ']),
            #return_ty
        )),
        (None, _) => quote!(format!("() => {}", #return_ty)),
    };

    Ok(ParsedFn {
        args_struct,
        derived_fn: DerivedTS {
            crate_rename,
            ts_name,
            docs,
            inline,
            inline_flattened: None,
            dependencies,
            export: true,
            export_to,
            concrete: HashMap::default(),
            bound: None,
        },
    })
}
