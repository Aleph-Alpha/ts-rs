use quote::quote;
use syn::{FieldsUnnamed, Result};

use crate::attr::{FieldAttr, Inflection};
use crate::DerivedTS;

pub(crate) fn newtype(
    name: &str,
    rename_all: &Option<Inflection>,
    fields: &FieldsUnnamed,
) -> Result<DerivedTS> {
    if rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to newtype structs");
    }
    let inner = fields.unnamed.first().unwrap();
    let FieldAttr {
        type_override,
        rename: rename_inner,
        inline,
        skip,
        flatten,
    } = FieldAttr::from_attrs(&inner.attrs)?;

    match (&rename_inner, skip, flatten) {
        (Some(_), _, _) => syn_err!("`rename` is not applicable to newtype fields"),
        (_, true, _) => syn_err!("`skip` is not applicable to newtype fields"),
        (_, _, true) => syn_err!("`flatten` is not applicable to newtype fields"),
        _ => {}
    };

    let inner_ty = &inner.ty;
    let inline_def = match &type_override {
        Some(o) => quote!(#o),
        None if inline => quote!(<#inner_ty as ts_rs::TS>::inline(0)),
        None => quote!(<#inner_ty as ts_rs::TS>::name()),
    };
    Ok(DerivedTS {
        decl: quote!(format!("type {} = {};", #name, #inline_def)),
        inline: inline_def,
        inline_flattened: None,
        name: name.to_owned(),
        dependencies: match (inline, &type_override) {
            (_, Some(_)) => quote!(vec![]),
            (true, _) => quote! {
                <#inner_ty as ts_rs::TS>::dependencies()
            },
            (false, _) => quote! {
                match <#inner_ty as ts_rs::TS>::transparent() {
                    true => <#inner_ty as ts_rs::TS>::dependencies(),
                    false => vec![(std::any::TypeId::of::<#inner_ty>(), <#inner_ty as ts_rs::TS>::name())]
                }
            },
        },
    })
}
