#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
#[ts(rename_all = "UPPERCASE")]
struct Rename {
    a: i32,
    b: i32,
}

#[derive(serde::Serialize, TS)]
struct RenameSerdeSpecialChar {
    #[serde(rename = "a/b")]
    b: i32,
}

#[test]
fn test() {
    assert_eq!(Rename::inline(), "{ A: number, B: number, }");
    assert_eq!(RenameSerdeSpecialChar::inline(), r#"{ "a/b": number, }"#);
}
