#![macro_use]
#![deny(unused)]

use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_quote, spanned::Spanned, ConstParam, GenericParam, Generics, Item, LifetimeParam, Result,
    Type, TypeArray, TypeParam, TypeParen, TypePath, TypeReference, TypeSlice, TypeTuple,
    WhereClause,
};

use crate::{deps::Dependencies, utils::format_generics};

#[macro_use]
mod utils;
mod attr;
mod deps;
mod types;

struct DerivedTS {
    ts_name: String,
    docs: String,
    inline: TokenStream,
    inline_flattened: Option<TokenStream>,
    dependencies: Dependencies,
    concrete: HashMap<Ident, Type>,

    export: bool,
    export_to: Option<String>,
}

impl DerivedTS {
    fn into_impl(mut self, rust_ty: Ident, generics: Generics) -> TokenStream {
        let export = self
            .export
            .then(|| self.generate_export_test(&rust_ty, &generics));

        let output_path_fn = {
            let path = match self.export_to.as_deref() {
                Some(dirname) if dirname.ends_with('/') => {
                    format!("{}{}.ts", dirname, self.ts_name)
                }
                Some(filename) => filename.to_owned(),
                None => format!("{}.ts", self.ts_name),
            };

            quote! {
                fn output_path() -> Option<&'static std::path::Path> {
                    Some(std::path::Path::new(#path))
                }
            }
        };

        let docs = match &*self.docs {
            "" => None,
            docs => Some(quote!(const DOCS: Option<&'static str> = Some(#docs);)),
        };

        let ident = self.ts_name.clone();
        let impl_start = generate_impl_block_header(&rust_ty, &generics, &self.dependencies);
        let assoc_type = generate_assoc_type(&rust_ty, &generics, &self.concrete);
        let name = self.generate_name_fn(&generics);
        let inline = self.generate_inline_fn();
        let decl = self.generate_decl_fn(&rust_ty, &generics);
        let dependencies = &self.dependencies;
        let generics_fn = self.generate_generics_fn(&generics);

        quote! {
            #impl_start {
                #assoc_type

                fn ident() -> String {
                    #ident.to_owned()
                }

                #docs
                #name
                #decl
                #inline
                #generics_fn
                #output_path_fn

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
    fn name_with_generics(&self, generics: &Generics) -> TokenStream {
        let name = &self.ts_name;
        let mut generics_ts_names = generics
            .type_params()
            .filter(|ty| !self.concrete.contains_key(&ty.ident))
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
    /// # Example:
    /// ```compile_fail
    /// struct Generic<A, B, const C: usize> { /* ... */ }
    /// ```
    /// has two generic type parameters, `A` and `B`. This function will therefore generate
    /// ```compile_fail
    /// struct A;
    /// impl ts_rs::TS for A { /* .. */ }
    ///
    /// struct B;
    /// impl ts_rs::TS for B { /* .. */ }
    /// ```
    fn generate_generic_types(&self, generics: &Generics) -> TokenStream {
        let generics = generics
            .type_params()
            .filter(|ty| !self.concrete.contains_key(&ty.ident))
            .map(|ty| ty.ident.clone());

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
                    fn inline() -> String { panic!("{} cannot be inlined", Self::name()) }
                    fn inline_flattened() -> String { panic!("{} cannot be flattened", Self::name()) }
                    fn decl() -> String { panic!("{} cannot be declared", Self::name()) }
                    fn decl_concrete() -> String { panic!("{} cannot be declared", Self::name()) }
                }
            )*
        }
    }

    fn generate_export_test(&self, rust_ty: &Ident, generics: &Generics) -> TokenStream {
        let test_fn = format_ident!(
            "export_bindings_{}",
            rust_ty.to_string().to_lowercase().replace("r#", "")
        );
        let generic_params = generics
            .type_params()
            .map(|ty| match self.concrete.get(&ty.ident) {
                None => quote! { ts_rs::Dummy },
                Some(ty) => quote! { #ty },
            });
        let ty = quote!(<#rust_ty<#(#generic_params),*> as ts_rs::TS>);

        quote! {
            #[cfg(test)]
            #[test]
            fn #test_fn() {
                #ty::export_all().expect("could not export type");
            }
        }
    }

    fn generate_generics_fn(&self, generics: &Generics) -> TokenStream {
        let generics = generics
            .type_params()
            .filter(|ty| !self.concrete.contains_key(&ty.ident))
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

    fn generate_name_fn(&self, generics: &Generics) -> TokenStream {
        let name = self.name_with_generics(generics);
        quote! {
            fn name() -> String {
                #name
            }
        }
    }

    fn generate_inline_fn(&self) -> TokenStream {
        let inline = &self.inline;

        let inline_flattened = self.inline_flattened.as_ref().map_or_else(
            || {
                quote! {
                    fn inline_flattened() -> String {
                        panic!("{} cannot be flattened", Self::name())
                    }
                }
            },
            |inline_flattened| {
                quote! {
                    fn inline_flattened() -> String {
                        #inline_flattened
                    }
                }
            },
        );
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
    fn generate_decl_fn(&mut self, rust_ty: &Ident, generics: &Generics) -> TokenStream {
        let name = &self.ts_name;
        let generic_types = self.generate_generic_types(generics);
        let ts_generics = format_generics(&mut self.dependencies, generics, &self.concrete);

        use GenericParam as G;
        // These are the generic parameters we'll be using.
        let generic_idents = generics.params.iter().filter_map(|p| match p {
            G::Lifetime(_) => None,
            G::Type(TypeParam { ident, .. }) => match self.concrete.get(ident) {
                // Since we named our dummy types the same as the generic parameters, we can just keep
                // the identifier of the generic parameter - its name is shadowed by the dummy struct.
                None => Some(quote!(#ident)),
                // If the type parameter is concrete, we use the type the user provided using
                // `#[ts(concrete)]`
                Some(concrete) => Some(quote!(#concrete)),
            },
            // We keep const parameters as they are, since there's no sensible default value we can
            // use instead. This might be something to change in the future.
            G::Const(ConstParam { ident, .. }) => Some(quote!(#ident)),
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

fn generate_assoc_type(
    rust_ty: &Ident,
    generics: &Generics,
    concrete: &HashMap<Ident, Type>,
) -> TokenStream {
    use GenericParam as G;

    let generics_params = generics.params.iter().map(|x| match x {
        G::Type(ty) => match concrete.get(&ty.ident) {
            None => quote! { ts_rs::Dummy },
            Some(ty) => quote! { #ty },
        },
        G::Const(ConstParam { ident, .. }) => quote! { #ident },
        G::Lifetime(LifetimeParam { lifetime, .. }) => quote! { #lifetime },
    });

    quote! { type WithoutGenerics = #rust_ty<#(#generics_params),*>; }
}

// generate start of the `impl TS for #ty` block, up to (excluding) the open brace
fn generate_impl_block_header(
    ty: &Ident,
    generics: &Generics,
    dependencies: &Dependencies,
) -> TokenStream {
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

    let where_bound = generate_where_clause(generics, dependencies);
    quote!(impl <#(#bounds),*> ts_rs::TS for #ty <#(#type_args),*> #where_bound)
}

fn generate_where_clause(generics: &Generics, dependencies: &Dependencies) -> WhereClause {
    let used_types = {
        let is_type_param = |id: &Ident| generics.type_params().any(|p| &p.ident == id);

        let mut used_types = HashSet::new();
        for ty in &dependencies.types {
            used_type_params(&mut used_types, ty, is_type_param);
        }
        used_types.into_iter()
    };

    let existing = generics.where_clause.iter().flat_map(|w| &w.predicates);
    parse_quote! {
        where #(#existing,)* #(#used_types: ts_rs::TS),*
    }
}

// Extracts all type parameters which are used within the given type.
// Associated types of a type parameter are extracted as well.
// Note: This will not extract `I` from `I::Item`, but just `I::Item`!
fn used_type_params<'ty, 'out>(
    out: &'out mut HashSet<&'ty Type>,
    ty: &'ty Type,
    is_type_param: impl Fn(&'ty Ident) -> bool + Copy + 'out,
) {
    use syn::{
        AngleBracketedGenericArguments as GenericArgs, GenericArgument as G, PathArguments as P,
    };

    match ty {
        Type::Array(TypeArray { elem, .. })
        | Type::Paren(TypeParen { elem, .. })
        | Type::Reference(TypeReference { elem, .. })
        | Type::Slice(TypeSlice { elem, .. }) => used_type_params(out, elem, is_type_param),
        Type::Tuple(TypeTuple { elems, .. }) => elems
            .iter()
            .for_each(|elem| used_type_params(out, elem, is_type_param)),
        Type::Path(TypePath { qself: None, path }) => {
            let first = path.segments.first().unwrap();
            if is_type_param(&first.ident) {
                // The type is either a generic parameter (e.g `T`), or an associated type of that
                // generic parameter (e.g `I::Item`). Either way, we return it.
                out.insert(ty);
                return;
            }

            let last = path.segments.last().unwrap();
            if let P::AngleBracketed(GenericArgs { ref args, .. }) = last.arguments {
                for generic in args {
                    if let G::Type(ty) = generic {
                        used_type_params(out, ty, is_type_param);
                    }
                }
            }
        }
        _ => (),
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
