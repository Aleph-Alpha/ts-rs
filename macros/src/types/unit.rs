use quote::quote;
use syn::Result;

use crate::{attr::StructAttr, deps::Dependencies, DerivedTS};

pub(crate) fn unit(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }

    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to unit structs");
    }

    Ok(DerivedTS {
        inline: quote!("null".to_owned()),
        decl: quote!(format!("type {} = null;", #name)),
        inline_flattened: None,
        name: name.to_owned(),
        dependencies: Dependencies::default(),
        export: attr.export,
        export_to: attr.export_to.clone(),
    })
}
