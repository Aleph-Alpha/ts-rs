use quote::quote;
use syn::{ItemStruct, Result};

use crate::attr::StructAttr;
use crate::DerivedTS;

pub(crate) fn unit(s: &ItemStruct) -> Result<DerivedTS> {
    let StructAttr { rename_all, rename } = StructAttr::from_attrs(&s.attrs)?;
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }
    let name = rename.unwrap_or_else(|| s.ident.to_string());

    Ok(DerivedTS {
        format: quote!("null".into()),
        decl: quote!(format!("export type {} = null;", #name)),
        flatten: None,
        name,
    })
}
