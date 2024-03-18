use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

#[derive(Default)]
pub struct Dependencies {
    dependencies: Vec<TokenStream>,
    pub types: Vec<Type>,
}

impl Dependencies {
    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        self.dependencies
            .push(quote![.extend(<#ty as ::ts_rs::TS>::dependency_types())]);
        self.types.push(ty.clone());
    }

    /// Adds the given type.
    pub fn push(&mut self, ty: &Type) {
        self.dependencies.push(quote![.push::<#ty>()]);
        self.dependencies.push(quote![
            .extend(<#ty as ::ts_rs::TS>::generics())
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
        let lines = &self.dependencies;
        tokens.extend(quote![{
            use ::ts_rs::typelist::TypeList;
            ()#(#lines)*
        }])
    }
}
