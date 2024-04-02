use quote::quote;
use syn::{Result, Type};

use crate::{
    attr::{EnumAttr, StructAttr, ContainerAttr},
    deps::Dependencies,
    DerivedTS,
};

pub(crate) fn type_as_struct(attr: &StructAttr, name: &str, type_as: &Type) -> Result<DerivedTS> {
    let crate_rename = attr.crate_rename();

    Ok(DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!(#type_as::inline()),
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

pub(crate) fn type_as_enum(attr: &EnumAttr, name: &str, type_as: &Type) -> Result<DerivedTS> {
    let crate_rename = attr.crate_rename();

    Ok(DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!(#type_as::inline()),
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
