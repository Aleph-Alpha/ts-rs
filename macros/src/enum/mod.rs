use quote::quote;
use syn::{ItemEnum, Result};

use crate::attr::FieldAttr;
use crate::DerivedTS;

pub(crate) fn r#enum(s: &ItemEnum) -> Result<DerivedTS> {
    let name = s.ident.to_string();
    let variants = s
        .variants
        .iter()
        .map(|variant| {
            let FieldAttr {
                type_override,
                rename,
                inline,
            } = FieldAttr::from_attrs(&variant.attrs)?;
            let name = match rename {
                Some(rn) => rn,
                None => variant.ident.to_string(),
            };
            if type_override.is_some() {
                syn_err!("`type_override` is not applicable to enum variants")
            }
            if inline {
                syn_err!("`inline` is not applicable to enum variants")
            }
            Ok(format!("{:?}", name))
        })
        .collect::<Result<Vec<String>>>()?;

    Ok(DerivedTS {
        format: quote!(vec![#(#variants),*].join(" | ")),
        decl: quote!(format!("export type {} = {};", #name, Self::format(0, true))),
        name,
    })
}
