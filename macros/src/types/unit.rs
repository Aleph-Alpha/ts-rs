use quote::quote;
use syn::{Result};

use crate::attr::{Inflection};
use crate::DerivedTS;

pub(crate) fn unit(name: &String, rename_all: &Option<Inflection>) -> Result<DerivedTS> {
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }

    Ok(DerivedTS {
        inline: quote!("null".to_owned()),
        decl: quote!(format!("export type {} = null;", #name)),
        inline_flattened: None,
        name: name.clone(),
        dependencies: quote!(vec![]),
    })
}
