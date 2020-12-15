use crate::DerivedTS;
use quote::quote;
use syn::{ItemStruct, Result};

pub(crate) fn unit(s: &ItemStruct) -> Result<DerivedTS> {
    let name = s.ident.to_string();

    Ok(DerivedTS {
        format: quote! {
            "null".into()
        },
        decl: quote! {
            format!("export type {} = null;", #name)
        },
        name,
    })
}
