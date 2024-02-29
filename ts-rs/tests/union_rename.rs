#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "tests-out/union_rename/")]
#[ts(rename_all = "lowercase", rename = "SimpleEnum")]
enum RenamedEnum {
    #[ts(rename = "ASDF")]
    A,
    B,
    C,
}

#[test]
fn test_simple_enum() {
    assert_eq!(
        RenamedEnum::decl(),
        r#"type SimpleEnum = "ASDF" | "b" | "c";"#
    )
}
