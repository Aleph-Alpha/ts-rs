use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

#[derive(Default)]
pub struct Dependencies(Vec<TokenStream>);

impl Dependencies {
    /// Adds all dependencies from the given type
    pub fn append_from(&mut self, ty: &Type) {
        self.0
            .push(quote!(dependencies.append(&mut <#ty as ts_rs::TS>::dependencies());));
    }

    /// Adds the given type if it's *not* transparent.
    /// If it is, all it's child dependencies are added instead.
    pub fn push_or_append_from(&mut self, ty: &Type) {
        self.0.push(quote! {
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
        self.0.push(quote! {
            dependencies.append(&mut #other);
        })
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dependencies = &self.0;
        tokens.extend(quote! {
            {
                let mut dependencies = vec![];
                #( #dependencies )*
                dependencies
            }
        })
    }
}
