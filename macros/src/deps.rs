use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Path, Type};

pub struct Dependencies {
    dependencies: Vec<TokenStream>,
    crate_rename: Path,
    pub types: Vec<Type>,
}

impl Dependencies {
    pub fn new(crate_rename: Path) -> Self {
        Self {
            dependencies: Vec::default(),
            crate_rename,
            types: Vec::default(),
        }
    }
    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        let crate_rename = &self.crate_rename;
        self.dependencies
            .push(quote![.extend(<#ty as #crate_rename::TS>::dependency_types())]);
        self.types.push(ty.clone());
    }

    /// Adds the given type.
    pub fn push(&mut self, ty: &Type) {
        let crate_rename = &self.crate_rename;
        self.dependencies.push(quote![.push::<#ty>()]);
        self.dependencies.push(quote![
            .extend(<#ty as #crate_rename::TS>::generics())
        ]);
        self.types.push(ty.clone());
    }

    pub fn append(&mut self, mut other: Dependencies) {
        self.dependencies.push(quote![.extend(#other)]);
        self.types.append(&mut other.types);
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_rename = &self.crate_rename;
        let lines = &self.dependencies;
        tokens.extend(quote![{
            use #crate_rename::typelist::TypeList;
            ()#(#lines)*
        }])
    }
}
