use quote::quote;
use syn::Expr;

use crate::{
    attr::{ContainerAttr, StructAttr},
    deps::Dependencies,
    DerivedTS,
};

pub(crate) fn empty_object(attr: &StructAttr, ts_name: Expr) -> DerivedTS {
    let crate_rename = attr.crate_rename();

    DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("Record<symbol, never>".to_owned()),
        inline_flattened: Some(quote!("{ }".to_owned())),
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name,
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
        ts_enum: None,
        is_enum: quote!(false),
    }
}

pub(crate) fn empty_array(attr: &StructAttr, ts_name: Expr) -> DerivedTS {
    let crate_rename = attr.crate_rename();

    DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("never[]".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name,
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
        ts_enum: None,
        is_enum: quote!(false),
    }
}

pub(crate) fn null(attr: &StructAttr, ts_name: Expr) -> DerivedTS {
    let crate_rename = attr.crate_rename();

    DerivedTS {
        crate_rename: crate_rename.clone(),
        inline: quote!("null".to_owned()),
        inline_flattened: Some(quote!("{ }".to_owned())),
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name,
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
        ts_enum: None,
        is_enum: quote!(false),
    }
}
