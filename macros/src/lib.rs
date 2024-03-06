#![macro_use]
#![deny(unused)]

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_quote, spanned::Spanned, ConstParam, GenericParam, Generics, Item, LifetimeParam, Result,
    TypeParam, WhereClause,
};

use crate::{deps::Dependencies, utils::format_generics};

#[macro_use]
mod utils;
mod attr;
mod deps;
mod types;

struct DerivedTS {
    generics: Generics,
    ts_name: String,
    docs: String,
    inline: TokenStream,
    inline_flattened: Option<TokenStream>,
    dependencies: Dependencies,

    export: bool,
    export_to: Option<String>,
}

impl DerivedTS {
    fn into_impl(mut self, rust_ty: Ident, generics: Generics) -> TokenStream {
        let export = self
            .export
            .then(|| self.generate_export_test(&rust_ty, &generics));

        let export_dir = std::option_env!("TS_RS_EXPORT_DIR");
        let export_to = {
            let path = match self.export_to.as_deref() {
                Some(dirname) if dirname.ends_with('/') => {
                    export_dir
                        .map_or_else(
                            || format!("{}{}.ts", dirname, self.ts_name),
                            |ts_rs_dir| format!(
                                "{}/{}{}.ts",
                                ts_rs_dir.trim_end_matches('/'),
                                dirname,
                                self.ts_name,
                            )
                        )
                },
                Some(filename) => {
                    export_dir
                        .map_or_else(
                            || filename.to_owned(),
                            |ts_rs_dir| format!(
                                "{}/{}",
                                ts_rs_dir.trim_end_matches('/'),
                                filename,
                            ),
                        )
                },
                None => {
                    export_dir
                        .map_or_else(
                            || format!("bindings/{}.ts", self.ts_name),
                            |ts_rs_dir| format!(
                                "{}/{}.ts",
                                ts_rs_dir.trim_end_matches('/'),
                                self.ts_name,
                            ),
                        )
                },
            };

            quote! {
                const EXPORT_TO: Option<&'static str> = Some(#path);
            }
        };

        let docs = match &*self.docs {
            "" => None,
            docs => Some(quote!(const DOCS: Option<&'static str> = Some(#docs);)),
        };

        let ident = self.ts_name.clone();
        let impl_start = generate_impl_block_header(&rust_ty, &generics);
        let assoc_type = generate_assoc_type(&rust_ty, &generics);
        let name = self.generate_name_fn();
        let inline = self.generate_inline_fn();
        let decl = self.generate_decl_fn(&rust_ty);
        let dependencies = &self.dependencies;
        let generics_fn = self.generate_generics_fn();

        quote! {
            #impl_start {
                #assoc_type
                #export_to

                fn ident() -> String {
                    #ident.to_owned()
                }

                #docs
                #name
                #decl
                #inline
                #generics_fn

                #[allow(clippy::unused_unit)]
                fn dependency_types() -> impl ts_rs::typelist::TypeList
                where
                    Self: 'static,
                {
                    #dependencies
                }
            }

            #export
        }
    }

    /// Returns an expression which evaluates to the TypeScript name of the type, including generic
    /// parameters.
    fn name_with_generics(&self) -> TokenStream {
        let name = &self.ts_name;
        let mut generics_ts_names = self
            .generics
            .type_params()
            .map(|ty| &ty.ident)
            .map(|generic| quote!(<#generic as ts_rs::TS>::name()))
            .peekable();

        if generics_ts_names.peek().is_some() {
            quote! {
                format!("{}<{}>", #name, vec![#(#generics_ts_names),*].join(", "))
            }
        } else {
            quote!(#name.to_owned())
        }
    }

    /// Generate a dummy unit struct for every generic type parameter of this type.
    /// Example:
    /// ```ignore
    /// struct Generic<A, B, const C: usize> { /* ... */ }
    /// ```
    /// has two generic type parameters, `A` and `B`. This function will therefor generate
    /// ```ignore
    /// struct A;
    /// impl ts_rs::TS for A { /* .. */ }
    ///
    /// struct B;
    /// impl ts_rs::TS for B { /* .. */ }
    /// ```
    fn generate_generic_types(&self) -> TokenStream {
        let generics = self.generics.type_params().map(|ty| ty.ident.clone());

        quote! {
            #(
                #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
                struct #generics;
                impl std::fmt::Display for #generics {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl TS for #generics {
                    type WithoutGenerics = #generics;
                    fn name() -> String { stringify!(#generics).to_owned() }
                }
            )*
        }
    }

    fn generate_export_test(&self, rust_ty: &Ident, generics: &Generics) -> TokenStream {
        let test_fn = format_ident!(
            "export_bindings_{}",
            rust_ty.to_string().to_lowercase().replace("r#", "")
        );
        let generic_params = generics.type_params().map(|_| quote! { ts_rs::Dummy });
        let ty = quote!(<#rust_ty<#(#generic_params),*> as ts_rs::TS>);

        quote! {
            #[cfg(test)]
            #[test]
            fn #test_fn() {
                #ty::export().expect("could not export type");
            }
        }
    }

    fn generate_generics_fn(&self) -> TokenStream {
        let generics = self
            .generics
            .type_params()
            .map(|TypeParam { ident, .. }| quote![.push::<#ident>().extend(<#ident as ts_rs::TS>::generics())]);
        quote! {
            #[allow(clippy::unused_unit)]
            fn generics() -> impl ts_rs::typelist::TypeList
            where
                Self: 'static,
            {
                use ts_rs::typelist::TypeList;
                ()#(#generics)*
            }
        }
    }

    fn generate_name_fn(&self) -> TokenStream {
        let name = self.name_with_generics();
        quote! {
            fn name() -> String {
                #name
            }
        }
    }

    fn generate_inline_fn(&self) -> TokenStream {
        let inline = &self.inline;

        let inline_flattened = self.inline_flattened.as_ref().map(|inline_flattened| {
            quote! {
                fn inline_flattened() -> String {
                    #inline_flattened
                }
            }
        });
        let inline = quote! {
            fn inline() -> String {
                #inline
            }
        };
        quote! {
            #inline
            #inline_flattened
        }
    }

    /// Generates the `decl()` and `decl_concrete()` methods.
    /// `decl_concrete()` is simple, and simply defers to `inline()`.
    /// For `decl()`, however, we need to change out the generic parameters of the type, replacing
    /// them with the dummy types generated by `generate_generic_types()`.
    fn generate_decl_fn(&mut self, rust_ty: &Ident) -> TokenStream {
        let name = &self.ts_name;
        let generic_types = self.generate_generic_types();
        let ts_generics = format_generics(&mut self.dependencies, &self.generics);

        use GenericParam as G;
        // These are the generic parameters we'll be using.
        let generic_idents = self.generics.params.iter().filter_map(|p| match p {
            G::Lifetime(_) => None,
            // Since we named our dummy types the same as the generic parameters, we can just keep
            // the identifier of the generic parameter - its name is shadowed by the dummy struct.
            //
            // We keep const parameters as they are, since there's no sensible default value we can
            // use instead. This might be something to change in the future.
            G::Type(TypeParam { ident, .. }) | G::Const(ConstParam { ident, .. }) => {
                Some(quote!(#ident))
            }
        });
        quote! {
            fn decl_concrete() -> String {
                format!("type {} = {};", #name, Self::inline())
            }
            fn decl() -> String {
                #generic_types
                let inline = <#rust_ty<#(#generic_idents,)*> as ts_rs::TS>::inline();
                let generics = #ts_generics;
                format!("type {}{generics} = {inline};", #name)
            }
        }
    }
}

fn generate_assoc_type(rust_ty: &Ident, generics: &Generics) -> TokenStream {
    use GenericParam as G;

    let generics_params = generics
        .params
        .iter()
        .map(|x| match x {
            G::Type(_) => quote! { ts_rs::Dummy },
            G::Const(ConstParam { ident, .. }) => quote! { #ident },
            G::Lifetime(LifetimeParam { lifetime, .. }) => quote! { #lifetime },
        })
        .collect::<Vec<_>>();

    if generics_params.is_empty() {
        quote! { type WithoutGenerics = #rust_ty; }
    } else {
        quote! { type WithoutGenerics = #rust_ty<#(#generics_params),*>; }
    }
}

// generate start of the `impl TS for #ty` block, up to (excluding) the open brace
fn generate_impl_block_header(ty: &Ident, generics: &Generics) -> TokenStream {
    use GenericParam as G;

    let bounds = generics.params.iter().map(|param| match param {
        G::Type(TypeParam {
            ident,
            colon_token,
            bounds,
            ..
        }) => quote!(#ident #colon_token #bounds),
        G::Lifetime(LifetimeParam {
            lifetime,
            colon_token,
            bounds,
            ..
        }) => quote!(#lifetime #colon_token #bounds),
        G::Const(ConstParam {
            const_token,
            ident,
            colon_token,
            ty,
            ..
        }) => quote!(#const_token #ident #colon_token #ty),
    });
    let type_args = generics.params.iter().map(|param| match param {
        G::Type(TypeParam { ident, .. }) | G::Const(ConstParam { ident, .. }) => quote!(#ident),
        G::Lifetime(LifetimeParam { lifetime, .. }) => quote!(#lifetime),
    });

    let where_bound = add_ts_to_where_clause(generics);
    quote!(impl <#(#bounds),*> ts_rs::TS for #ty <#(#type_args),*> #where_bound)
}

fn add_ts_to_where_clause(generics: &Generics) -> Option<WhereClause> {
    let generic_types = generics
        .type_params()
        .map(|ty| ty.ident.clone())
        .collect::<Vec<_>>();
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
