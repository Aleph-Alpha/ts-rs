use quote::quote;
use syn::{Result, parse_quote};

use crate::{attr::StructAttr, deps::Dependencies, DerivedTS};

pub(crate) fn empty_object(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    check_attributes(attr)?;
    let crate_rename = attr
        .crate_rename
        .clone()
        .unwrap_or_else(|| parse_quote!(::ts_rs));

    Ok(DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("Record<string, never>".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

pub(crate) fn empty_array(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    check_attributes(attr)?;
    let crate_rename = attr
        .crate_rename
        .clone()
        .unwrap_or_else(|| parse_quote!(::ts_rs));

    Ok(DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("never[]".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

pub(crate) fn null(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    check_attributes(attr)?;
    let crate_rename = attr
        .crate_rename
        .clone()
        .unwrap_or_else(|| parse_quote!(::ts_rs));

    Ok(DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("null".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

fn check_attributes(attr: &StructAttr) -> Result<()> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }

    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to unit structs");
    }

    Ok(())
}
