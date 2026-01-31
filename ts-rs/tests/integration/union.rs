#![allow(dead_code)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "union/")]
enum SimpleEnum {
    #[ts(rename = "asdf")]
    A,
    B,
    C,
    r#D,
}

#[test]
fn test_empty() {
    #[derive(TS)]
    enum Empty {}
    let cfg = Config::from_env();
    assert_eq!(Empty::decl(&cfg), "type Empty = never;")
}

#[test]
fn test_simple_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        SimpleEnum::decl(&cfg),
        r#"type SimpleEnum = "asdf" | "B" | "C" | "D";"#
    )
}
