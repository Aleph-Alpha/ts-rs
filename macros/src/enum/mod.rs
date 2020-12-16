use quote::quote;
use syn::{ItemEnum, Result};

use crate::attr::{EnumAttr, FieldAttr};
use crate::DerivedTS;

pub(crate) fn r#enum(s: &ItemEnum) -> Result<DerivedTS> {
    let EnumAttr { rename_all, rename } = EnumAttr::from_attrs(&s.attrs)?;

    let name = rename.unwrap_or_else(|| s.ident.to_string());
    let variants = s
        .variants
        .iter()
        .map(|variant| {
            let FieldAttr {
                type_override,
                rename,
                inline,
                skip,
                flatten,
            } = FieldAttr::from_attrs(&variant.attrs)?;
            if skip {
                return Ok(None)
            }
            let name = match (rename, &rename_all) {
                (Some(rn), _) => rn,
                (None, None) => variant.ident.to_string(),
                (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
            };
            if type_override.is_some() {
                syn_err!("`type_override` is not applicable to enum variants")
            }
            if inline {
                syn_err!("`inline` is not applicable to enum variants")
            }
            if flatten {
                syn_err!("`flatten` is not applicable to enum variants")
            }
            Ok(Some(format!("{:?}", name)))
        })
        .flat_map(|x| match x {
            Ok(Some(x)) => Some(Ok(x)),
            Err(err) => Some(Err(err)),
            Ok(None) => None,
        })
        .collect::<Result<Vec<String>>>()?;

    Ok(DerivedTS {
        format: quote!(vec![#(#variants),*].join(" | ")),
        decl: quote!(format!("export type {} = {};", #name, Self::format(0, true))),
        flatten: None,
        name,
    })
}
