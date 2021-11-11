#![allow(dead_code, clippy::blacklisted_name)]

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[serde(tag = "type")]
enum EnumWithInternalTag {
    A { foo: String },
    B { bar: i32 },
}

#[derive(Serialize, TS)]
struct InnerA {
    foo: String,
}

#[derive(Serialize, TS)]
struct InnerB {
    bar: i32,
}

#[derive(Serialize, TS)]
#[serde(tag = "type")]
enum EnumWithInternalTag2 {
    A(InnerA),
    B(InnerB),
}

#[test]
#[cfg(feature = "serde-compat")]
fn test_enums_with_internal_tags() {
    assert_eq!(
        EnumWithInternalTag::decl(),
        r#"type EnumWithInternalTag = { type: "A", foo: string, } | { type: "B", bar: number, };"#
    );

    assert_eq!(
        EnumWithInternalTag2::decl(),
        r#"type EnumWithInternalTag2 = { type: "A" } & InnerA | { type: "B" } & InnerB;"#
    );
}
