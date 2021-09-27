use quote::quote;
use syn::Result;

use crate::{attr::Inflection, DerivedTS};

pub(crate) fn unit(name: &str, rename_all: &Option<Inflection>) -> Result<DerivedTS> {
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }

    Ok(DerivedTS {
        inline: quote!("null".to_owned()),
        decl: quote!(format!("type {} = null;", #name)),
        inline_flattened: None,
        name: name.to_owned(),
        dependencies: quote!(vec![]),
    })
}
