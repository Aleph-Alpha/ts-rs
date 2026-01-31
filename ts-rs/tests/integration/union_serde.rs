#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::{Deserialize, Serialize};
use ts_rs::{Config, TS};

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "kind", content = "d"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "kind", content = "d"))]
#[ts(export, export_to = "union_serde/")]
enum SimpleEnum {
    A,
    B,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "kind", content = "data"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "kind", content = "data"))]
#[ts(export, export_to = "union_serde/")]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    T(i32, SimpleEnum),
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), ts(untagged))]
#[ts(export, export_to = "union_serde/")]
enum Untagged {
    Foo(String),
    Bar(i32),
    None,
}

#[test]
fn test_serde_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        SimpleEnum::decl(&cfg),
        r#"type SimpleEnum = { "kind": "A" } | { "kind": "B" };"#
    );
    assert_eq!(
        ComplexEnum::decl(&cfg),
        r#"type ComplexEnum = { "kind": "A" } | { "kind": "B", "data": { foo: string, bar: number, } } | { "kind": "W", "data": SimpleEnum } | { "kind": "F", "data": { nested: SimpleEnum, } } | { "kind": "T", "data": [number, SimpleEnum] };"#
    );

    assert_eq!(
        Untagged::decl(&cfg),
        r#"type Untagged = string | number | null;"#
    )
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(
    feature = "serde-compat",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
enum Enum {
    FirstOption,
    SecondOption,
}

#[test]
fn test_rename_all() {
    assert_eq!(Enum::inline(), r#""firstOption" | "secondOption""#);
}
