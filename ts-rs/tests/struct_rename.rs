#![allow(dead_code)]

use ts_rs::TS;

#[test]
fn rename_all() {
    #[derive(TS)]
    #[ts(rename_all = "UPPERCASE")]
    struct Rename {
        a: i32,
        b: i32,
    }

    assert_eq!(Rename::inline(), "{ A: number, B: number, }");
}

#[test]
fn rename_all_camel_case() {
    #[derive(TS)]
    #[ts(rename_all = "camelCase")]
    struct Rename {
        crc32c_hash: i32,
        b: i32,
    }

    assert_eq!(Rename::inline(), "{ crc32cHash: number, b: number, }");
}

#[test]
fn rename_all_pascal_case() {
    #[derive(TS)]
    #[ts(rename_all = "PascalCase")]
    struct Rename {
        crc32c_hash: i32,
        b: i32,
    }

    assert_eq!(Rename::inline(), "{ Crc32cHash: number, B: number, }");
}

#[cfg(feature = "serde-compat")]
#[test]
fn serde_rename_special_char() {
    #[derive(serde::Serialize, TS)]
    struct RenameSerdeSpecialChar {
        #[serde(rename = "a/b")]
        b: i32,
    }

    assert_eq!(RenameSerdeSpecialChar::inline(), r#"{ "a/b": number, }"#);
}
