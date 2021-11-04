#![allow(dead_code)]

use serde::Deserialize;
use ts_rs::TS;

#[derive(TS, Deserialize)]
#[serde(tag = "kind", content = "d")]
enum SimpleEnum {
    A,
    B,
}

#[derive(TS, Deserialize)]
#[serde(tag = "kind", content = "data")]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    T(i32, SimpleEnum),
}

#[derive(TS, Deserialize)]
#[serde(untagged)]
enum Untagged {
    Foo(String),
    Bar(i32),
    None,
}

#[cfg(feature = "serde-compat")]
#[test]
fn test_serde_enum() {
    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = { kind: "A" } | { kind: "B" };"#
    );
    assert_eq!(
        ComplexEnum::decl(),
        r#"type ComplexEnum = { kind: "A" } | { kind: "B", data: { foo: string, bar: number, } } | { kind: "W", data: SimpleEnum } | { kind: "F", data: { nested: SimpleEnum, } } | { kind: "T", data: [number, SimpleEnum] };"#
    );

    assert_eq!(
        Untagged::decl(),
        r#"type Untagged = string | number | null;"#
    )
}
