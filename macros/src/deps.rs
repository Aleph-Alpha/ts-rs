use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Dependencies {
    v0: Vec<TokenStream>,
    v1: Vec<TokenStream>
}

impl Default for Dependencies {
    fn default() -> Self {
        Dependencies {
            v0: Vec::new(),
            v1: vec![quote![()]]
        }
    }
}

impl Dependencies {
    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        self.v1.push(quote![.extend(<#ty as ts_rs::TS>::dependency_types())]);
        self.v0
            .push(quote!(dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());));
    }

    /// Adds the given type if it's *not* transparent.
    /// If it is, all it's child dependencies are added instead.
    pub fn push_or_append_from(&mut self, ty: &Type) {
        self.v1.push(quote![.push::<#ty>()]);
        self.v0.push(quote! {
            if <#ty as ts_rs::TS>::transparent() {
              dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());
            } else {
                if let Some(dep) = ts_rs::Dependency::from_ty::<#ty>() {
                    dependencies.push(dep);
                }
            }
        });
    }

    pub fn append(&mut self, other: Dependencies) {
        let other_v2 = other.to_tokens_v2();
        self.v1.push(quote![.extend(#other_v2)]);
        self.v0.push(quote! {
            dependencies.append(&mut #other);
        })
    }

    pub fn to_tokens_v2(&self) -> impl ToTokens + '_ {
        struct ToTokensV2<'a>(&'a Dependencies);
        impl<'a> ToTokens for ToTokensV2<'a> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let lines = &self.0.v1;
                tokens.extend(quote![{
                    use ts_rs::typelist::TypeList;
                    #(#lines)*
                }])
            }
        }
        ToTokensV2(self)
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dependencies = &self.v0;
        tokens.extend(quote! {
            {
                let mut dependencies = Vec::new();
                #( #dependencies )*
                dependencies
            }
        })
    }
}
