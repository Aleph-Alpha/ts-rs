#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
enum SimpleEnum {
    A(String),
    B(usize),
    C,
}

#[test]
fn test_simple_enum() {
    assert_eq!(
        SimpleEnum::decl(),
        r#"export type SimpleEnum = string | usize | "C" ;"#
    )
}
