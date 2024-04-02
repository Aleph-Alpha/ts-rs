use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Path, Type};

pub struct Dependencies {
    crate_rename: Path,
    dependencies: HashSet<Dependency>,
    pub types: Vec<Type>,
}

#[derive(Hash, Eq, PartialEq)]
enum Dependency {
    DependencyTypes { crate_rename: Path, ty: Type },
    Generics { crate_rename: Path, ty: Type },
    Type(Type),
}

impl Dependencies {
    pub fn new(crate_rename: Path) -> Self {
        Self {
            dependencies: HashSet::default(),
            crate_rename,
            types: Vec::default(),
        }
    }

    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        self.dependencies.insert(Dependency::DependencyTypes {
            crate_rename: self.crate_rename.clone(),
            ty: ty.clone(),
        });

        self.types.push(ty.clone());
    }

    /// Adds the given type.
    pub fn push(&mut self, ty: &Type) {
        self.dependencies.insert(Dependency::Type(ty.clone()));
        self.dependencies.insert(Dependency::Generics {
            crate_rename: self.crate_rename.clone(),
            ty: ty.clone(),
        });
        self.types.push(ty.clone());
    }

    pub fn append(&mut self, mut other: Dependencies) {
        self.dependencies.extend(other.dependencies);

        if !other.types.is_empty() {
            self.types.append(&mut other.types);
        }
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_rename = &self.crate_rename;
        let lines = self.dependencies.iter();

        tokens.extend(quote![{
            use #crate_rename::typelist::TypeList;
            ()#(#lines)*
        }]);
    }
}

impl ToTokens for Dependency {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Dependency::DependencyTypes { crate_rename, ty } => {
                quote![.extend(<#ty as #crate_rename::TS>::dependency_types())]
            }
            Dependency::Generics { crate_rename, ty } => {
                quote![.extend(<#ty as #crate_rename::TS>::generics())]
            }
            Dependency::Type(ty) => quote![.push::<#ty>()],
        });
    }
}
