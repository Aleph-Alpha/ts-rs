#![allow(non_snake_case)]
#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
#[ts(
    export,
    export_to = "tests-out/struct_rename/",
    rename_all = "UPPERCASE"
)]
struct RenameAllUpper {
    a: i32,
    b: i32,
}

#[test]
fn rename_all() {
    assert_eq!(RenameAllUpper::inline(), "{ A: number, B: number, }");
}

#[derive(TS)]
#[ts(
    export,
    export_to = "tests-out/struct_rename/",
    rename_all = "camelCase"
)]
struct RenameAllCamel {
    crc32c_hash: i32,
    b: i32,
    alreadyCamelCase: i32,
}

#[test]
fn rename_all_camel_case() {
    assert_eq!(
        RenameAllCamel::inline(),
        "{ crc32cHash: number, b: number, alreadyCamelCase: number, }"
    );
}

#[derive(TS)]
#[ts(
    export,
    export_to = "tests-out/struct_rename/",
    rename_all = "PascalCase"
)]
struct RenameAllPascal {
    crc32c_hash: i32,
    b: i32,
}

#[test]
fn rename_all_pascal_case() {
    assert_eq!(
        RenameAllPascal::inline(),
        "{ Crc32cHash: number, B: number, }"
    );
}

#[derive(serde::Serialize, TS)]
#[ts(
    export,
    export_to = "tests-out/struct_rename/",
    rename_all = "camelCase"
)]
struct RenameSerdeSpecialChar {
    #[serde(rename = "a/b")]
    b: i32,
}

#[cfg(feature = "serde-compat")]
#[test]
fn serde_rename_special_char() {
    assert_eq!(RenameSerdeSpecialChar::inline(), r#"{ "a/b": number, }"#);
}
