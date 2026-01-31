#![allow(dead_code, unused)]

use std::path::PathBuf;

use ts_rs::{Config, TS};

mod arrays;
mod bound;
mod bson;
mod chrono;
mod complex_flattened_type;
mod concrete_generic;
mod docs;
mod enum_flattening;
mod enum_flattening_nested;
mod enum_struct_rename_all;
mod enum_variant_annotation;
mod export_manually;
mod export_to;
mod field_rename;
mod flatten;
mod generic_fields;
mod generic_without_import;
mod generics;
mod generics_flatten;
mod hashmap;
mod hashset;
mod impl_primitive;
mod imports;
mod indexmap;
mod infer_as;
mod issue_168;
mod issue_232;
mod issue_308;
mod issue_317;
mod issue_338;
mod issue_397;
mod issue_415;
mod issue_70;
mod issue_80;
mod jiff;
mod leading_colon;
mod lifetimes;
mod list;
mod merge_same_file_imports;
mod nested;
mod optional_field;
mod path_bug;
mod ranges;
mod raw_idents;
mod recursion_limit;
mod references;
mod repr_enum;
mod same_file_export;
mod self_referential;
mod semver;
mod serde_json;
mod serde_skip_serializing;
mod serde_skip_with_default;
mod serde_with;
mod simple;
mod skip;
mod slices;
mod struct_rename;
mod struct_tag;
mod tokio;
mod top_level_type_as;
mod top_level_type_override;
mod tuple;
mod type_as;
mod type_override;
mod union;
mod union_named_serde_skip;
mod union_rename;
mod union_serde;
mod union_unnamed_serde_skip;
mod union_with_data;
mod union_with_internal_tag;
mod unit;
mod r#unsized;

// Returns the path to the file into which `T` is exported
fn target_file<T: TS>(cfg: &Config) -> PathBuf {
    cfg.out_dir().join(T::output_path().unwrap())
}

// Read the bindings for `T` from disk
fn read_file<T: TS>(cfg: &Config) -> String {
    std::fs::read_to_string(target_file::<T>(cfg)).unwrap()
}
