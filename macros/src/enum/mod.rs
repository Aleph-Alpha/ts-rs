use quote::quote;
use syn::{spanned::Spanned, Fields, ItemEnum, Result};

use crate::attr::{EnumAttr, FieldAttr};
use crate::DerivedTS;

pub(crate) fn r#enum(s: &ItemEnum) -> Result<DerivedTS> {
    let EnumAttr { rename_all, rename } = EnumAttr::from_attrs(&s.attrs)?;

    if let Some(v) = s
        .variants
        .iter()
        .find(|v| !matches!(v.fields, Fields::Unit))
    {
        syn_err!(v.span(); "variant has data attached. Such enums are not yet supported.");
    }

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
            match (skip, &type_override, inline, flatten) {
                (true, ..) => return Ok(None),
                (_, Some(_), ..) => syn_err!("`type_override` is not applicable to enum variants"),
                (_, _, true, ..) => syn_err!("`inline` is not applicable to enum variants"),
                (_, _, _, true) => syn_err!("`flatten` is not applicable to enum variants"),
                _ => {}
            };

            let name = match (rename, &rename_all) {
                (Some(rn), _) => rn,
                (None, None) => variant.ident.to_string(),
                (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
            };

            Ok(Some(format!("{:?}", name)))
        })
        .flat_map(|x| match x {
            Ok(Some(x)) => Some(Ok(x)),
            Err(err) => Some(Err(err)),
            Ok(None) => None,
        })
        .collect::<Result<Vec<String>>>()?;

    Ok(DerivedTS {
        inline: quote!(vec![#(#variants),*].join(" | ")),
        decl: quote!(format!("export type {} = {};", #name, Self::inline(0))),
        inline_flattened: None,
        name,
    })
}
