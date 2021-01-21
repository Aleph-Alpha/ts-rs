#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
enum SimpleEnum {
    A(String),
    B(i64),
    C,
    D(String, i64)
}

#[test]
fn test_simple_enum() {
    assert_eq!(
        SimpleEnum::decl(),
        r#"export type SimpleEnum = string | number | "C" | [string, number];"#
    )
}
