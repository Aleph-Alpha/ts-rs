#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
enum SimpleEnum {
    #[ts(rename = "asdf")]
    A,
    B,
    C,
}

#[test]
fn test_simple_enum() {
    assert_eq!(
        SimpleEnum::decl().unwrap(),
        r#"export type SimpleEnum = "asdf" | "B" | "C";"#
    )
}
