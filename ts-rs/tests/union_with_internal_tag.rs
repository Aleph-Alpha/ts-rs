#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[serde(tag = "type")]
enum EnumWithInternalTag {
    A { foo: String },
    B { bar: i32 },
}

#[test]
fn test_enum_with_internal_tag() {
    assert_eq!(
        EnumWithInternalTag::decl(),
        r#"type EnumWithInternalTag = {
    type: "A",
    foo: string,
} | {
    type: "B",
    bar: number,
};"#
    )
}
