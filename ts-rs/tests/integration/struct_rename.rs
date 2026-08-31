#![allow(non_snake_case)]
#![allow(dead_code)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "struct_rename/", rename_all = "UPPERCASE")]
struct RenameAllUpper {
    a: i32,
    b: i32,
}

#[test]
fn rename_all() {
    let cfg = Config::from_env();
    assert_eq!(RenameAllUpper::inline(&cfg), "{ A: number, B: number, }");
}

#[derive(TS)]
#[ts(export, export_to = "struct_rename/", rename_all = "camelCase")]
struct RenameAllCamel {
    crc32c_hash: i32,
    b: i32,
    alreadyCamelCase: i32,
}

#[test]
fn rename_all_camel_case() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameAllCamel::inline(&cfg),
        "{ crc32cHash: number, b: number, alreadyCamelCase: number, }"
    );
}

#[derive(TS)]
#[ts(export, export_to = "struct_rename/", rename_all = "PascalCase")]
struct RenameAllPascal {
    crc32c_hash: i32,
    b: i32,
}

#[test]
fn rename_all_pascal_case() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameAllPascal::inline(&cfg),
        "{ Crc32cHash: number, B: number, }"
    );
}

#[derive(TS, Default, serde::Serialize)]
#[ts(export, export_to = "struct_rename/")]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "SCREAMING-KEBAB-CASE"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "SCREAMING-KEBAB-CASE"))]
struct RenameAllScreamingKebab {
    crc32c_hash: i32,
    some_field: i32,
    some_other_field: i32,
}

#[test]
fn rename_all_screaming_kebab_case() {
    let rename_all = RenameAllScreamingKebab::default();
    let cfg = Config::from_env();
    assert_eq!(
        RenameAllScreamingKebab::inline(&cfg),
        r#"{ "CRC32C-HASH": number, "SOME-FIELD": number, "SOME-OTHER-FIELD": number, }"#
    );
}

#[derive(serde::Serialize, TS)]
#[ts(export, export_to = "struct_rename/", rename_all = "camelCase")]
struct RenameSerdeSpecialChar {
    #[serde(rename = "a/b")]
    b: i32,
}

#[cfg(feature = "serde-compat")]
#[test]
fn serde_rename_special_char() {
    let cfg = Config::from_env();
    assert_eq!(RenameSerdeSpecialChar::inline(&cfg), r#"{ "a/b": number, }"#);
}

// struct-level renames

#[derive(TS)]
#[ts(export, export_to = "struct_rename/")]
#[ts(rename = "RenamedWithStrLiteral")]
enum WithStrLiteral {
    A,
    B,
    C,
}

#[test]
fn test_rename_with_str_literal() {
    let cfg = Config::from_env();
    assert_eq!(
        WithStrLiteral::decl(&cfg),
        r#"type RenamedWithStrLiteral = "A" | "B" | "C";"#
    )
}

#[derive(TS)]
#[ts(export, export_to = "struct_rename/")]
#[ts(rename = format!("{}With{}", "Renamed", "StringExpression"))]
enum WithStringExpression {
    A,
    B,
    C,
}

#[test]
fn test_rename_with_string_expression() {
    let cfg = Config::from_env();
    assert_eq!(
        WithStringExpression::decl(&cfg),
        r#"type RenamedWithStringExpression = "A" | "B" | "C";"#
    )
}

#[derive(TS)]
#[ts(export, export_to = "struct_rename/")]
#[ts(rename = &"RenamedWithStrExpression")]
enum WithStrExpression {
    A,
    B,
    C,
}

#[test]
fn test_rename_with_str_expression() {
    let cfg = Config::from_env();
    assert_eq!(
        WithStrExpression::decl(&cfg),
        r#"type RenamedWithStrExpression = "A" | "B" | "C";"#
    )
}

#[derive(TS)]
#[ts(export, export_to = "struct_rename/")]
#[ts(rename = format!("i_am_inside_module_{}", module_path!().rsplit_once("::").unwrap().1))]
enum RenameUsingModuleName {
    A,
    B,
    C,
}

#[test]
fn test_rename_using_module_name() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameUsingModuleName::decl(&cfg),
        r#"type i_am_inside_module_struct_rename = "A" | "B" | "C";"#
    )
}
