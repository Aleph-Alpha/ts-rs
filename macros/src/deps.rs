use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

#[derive(Default)]
pub struct Dependencies(Vec<TokenStream>);

impl Dependencies {
    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        self.0
            .push(quote![.extend(<#ty as ts_rs::TS>::dependency_types())]);
    }

    /// Adds the given type.
    /// If the type is transparent, then we'll get resolve the child dependencies during runtime.
    pub fn push(&mut self, ty: &Type) {
        self.0.push(quote![.push::<#ty>()]);
    }

    pub fn append(&mut self, other: Dependencies) {
        self.0.push(quote![.extend(#other)]);
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lines = &self.0;
        tokens.extend(quote![{
            use ts_rs::typelist::TypeList;
            ()#(#lines)*
        }])
    }
}
