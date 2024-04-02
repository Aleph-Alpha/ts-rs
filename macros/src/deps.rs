use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use std::rc::Rc;
use syn::{Path, Type};

pub struct Dependencies {
    crate_rename: Path,

    // Types which are used in the Rust definition. This is tracked to generate a good `where`
    // bound later
    used_types: HashSet<Rc<Type>>,
    // Types on which the type has a dependency, making it show up in the resulting `TypeList`.
    direct_deps: HashSet<Rc<Type>>,
    // Types whose dependencies we depend on. The `dependency_types()` of every type in this set
    // is appended to the resulting `TypeList`
    transitive_deps: HashSet<Rc<Type>>,
    // Types whose generics we depend on. The `generics()` of every type in this set is appended
    // to the resulting `TypeList`
    generics_deps: HashSet<Rc<Type>>,
}

impl Dependencies {
    pub fn new(crate_rename: Path) -> Self {
        Self {
            crate_rename,
            used_types: Default::default(),
            direct_deps: Default::default(),
            transitive_deps: Default::default(),
            generics_deps: Default::default(),
        }
    }

    pub fn used_types(&self) -> impl Iterator<Item = &Type> {
        self.used_types.iter().map(Rc::as_ref)
    }

    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        let ty = Rc::new(ty.clone());
        self.used_types.insert(ty.clone());

        self.transitive_deps.insert(ty);
    }

    /// Adds the given type.
    pub fn push(&mut self, ty: &Type) {
        let ty = Rc::new(ty.clone());
        self.used_types.insert(ty.clone());

        self.direct_deps.insert(ty.clone());
        self.generics_deps.insert(ty.clone());
    }

    pub fn append(&mut self, other: Dependencies) {
        self.used_types.extend(other.used_types.into_iter());
        self.direct_deps.extend(other.direct_deps.into_iter());
        self.transitive_deps
            .extend(other.transitive_deps.into_iter());
        self.generics_deps.extend(other.generics_deps.into_iter());
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let rename = &self.crate_rename;

        let direct = self.direct_deps.iter().map(|ty| {
            quote! {
                .push::<#ty>()
            }
        });
        let transitive = self.transitive_deps.iter().map(|ty| {
            quote! {
                .extend(<#ty as #rename::TS>::dependency_types())
            }
        });
        let generics = self.generics_deps.iter().map(|ty| {
            quote! {
                .extend(<#ty as #rename::TS>::generics())
            }
        });

        tokens.extend(quote![{
            use #rename::typelist::TypeList;
            () #(#direct)* #(#transitive)* #(#generics)*
        }])
    }
}
