#![macro_use]
#![deny(unused)]

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, spanned::Spanned, GenericParam, Generics, Item, Result, WhereClause};

use crate::deps::Dependencies;

#[macro_use]
mod utils;
mod attr;
mod deps;
mod types;

struct DerivedTS {
    name: String,
    inline: TokenStream,
    decl: TokenStream,
    inline_flattened: Option<TokenStream>,
    dependencies: Dependencies,
    export: Option<Option<String>>,
}

impl DerivedTS {
    fn generate_export_test(
        &self,
        rust_ty: &Ident,
        generics: &Generics,
        path: &str,
    ) -> Option<TokenStream> {
        let test_fn = format_ident!("export_bindings_{}", &self.name.to_lowercase());
        let generic_params = generics
            .params
            .iter()
            .filter(|param| matches!(param, GenericParam::Type(_)))
            .map(|_| quote! { () });

        let rust_ty = quote!(#rust_ty<#(#generic_params),*>);
        let expanded = quote! {
            #[cfg(test)]
            #[test]
            fn #test_fn() {
                use std::{
                    path::Path,
                    fmt::Write,
                    collections::BTreeSet,
                    any::TypeId,
                };
                use ts_rs::export::{FmtCfg, fmt_ts};

                let path = Path::new(#path);
                let mut buffer = String::with_capacity(1024);

                let deps = <#rust_ty as ts_rs::TS>::dependencies()
                    .into_iter()
                    .filter(|dep| dep.type_id != TypeId::of::<#rust_ty>())
                    .collect::<BTreeSet<_>>();
                for dep in deps {
                    let exported_to = match dep.exported_to {
                        None => continue,
                        Some(to) => Path::new(to),
                    };
                    let rel_path = ts_rs::export::import_path(
                        path,
                        exported_to,
                    );
                    write!(&mut buffer, "import {{ {} }} from {:?};\n", &dep.ts_name, rel_path).unwrap();
                }
                buffer.push_str("\n");

                buffer.push_str("export ");
                buffer.push_str(
                   &<#rust_ty as ts_rs::TS>::decl()
                );

                let buffer = fmt_ts(
                    &path,
                    &buffer,
                    &FmtCfg::new().deno().build()
                )
                .expect("could not format output");

                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).expect("could not create directory");
                }
                std::fs::write(path, &buffer).expect("could not write bindings to file");
            }
        };
        Some(expanded)
    }

    fn into_impl(self, rust_ty: Ident, generics: Generics) -> TokenStream {
        let export_to = self
            .export
            .clone()
            .map(|export| export.unwrap_or_else(|| format!("bindings/{}.ts", &self.name)));
        let export = export_to
            .as_ref()
            .map(|to| self.generate_export_test(&rust_ty, &generics, to));
        let export_to = export_to
            .map(|to| quote!(Some(#to)))
            .unwrap_or_else(|| quote!(None));

        let DerivedTS {
            name,
            inline,
            decl,
            inline_flattened,
            dependencies,
            ..
        } = self;
        let inline_flattened = inline_flattened
            .map(|t| {
                quote! {
                    fn inline_flattened() -> String {
                        #t
                    }
                }
            })
            .unwrap_or_else(TokenStream::new);

        let Generics {
            ref lt_token,
            ref params,
            ref gt_token,
            where_clause: _,
        } = generics;

        let where_clause = add_ts_trait_bound(&generics);
        quote! {
            impl #lt_token #params #gt_token ts_rs::TS for #rust_ty #lt_token #params #gt_token #where_clause {
                const EXPORTED_TO: Option<&'static str> = #export_to;

                fn decl() -> String {
                    #decl
                }
                fn name() -> String {
                    #name.to_owned()
                }
                fn inline() -> String {
                    #inline
                }
                #inline_flattened
                fn dependencies() -> Vec<ts_rs::Dependency> {
                    #dependencies
                }
                fn transparent() -> bool {
                    false
                }
            }

            #export
        }
    }
}

fn add_ts_trait_bound(generics: &Generics) -> Option<WhereClause> {
    let generic_types: Vec<_> = generics
        .params
        .iter()
        .filter_map(|gp| match gp {
            GenericParam::Type(ty) => Some(ty.ident.clone()),
            _ => None,
        })
        .collect();
    if generic_types.is_empty() {
        return generics.where_clause.clone();
    }
    match generics.where_clause {
        None => Some(parse_quote! { where #( #generic_types : ts_rs::TS ),* }),
        Some(ref w) => {
            let bounds = w.predicates.iter();
            Some(parse_quote! { where #(#bounds,)* #( #generic_types : ts_rs::TS ),* })
        }
    }
}

/// Derives [TS](./trait.TS.html) for a struct or enum.
/// Please take a look at [TS](./trait.TS.html) for documentation.
#[proc_macro_derive(TS, attributes(ts))]
pub fn typescript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match entry(input) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}

fn entry(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let input = syn::parse::<Item>(input)?;
    let (ts, ident, generics) = match input {
        Item::Struct(s) => (types::struct_def(&s)?, s.ident, s.generics),
        Item::Enum(e) => (types::enum_def(&e)?, e.ident, e.generics),
        _ => syn_err!(input.span(); "unsupported item"),
    };

    Ok(ts.into_impl(ident, generics))
}
