#![allow(dead_code)]

use ts_rs::TS;
use serde::{Deserialize};


#[derive(TS, Deserialize)]
#[serde(tag="kind", content="d")]
enum SimpleEnum {
    A,
    B,
}

#[derive(TS, Deserialize)]
#[serde(tag="kind", content="data")]
enum ComplexEnum {
    A,
    B{foo: String, bar: f64},
    W(SimpleEnum)
}



#[test]
fn test_serde_enum() {
    assert_eq!(
        SimpleEnum::decl(),
r#"export type SimpleEnum = {kind: "A", d: null} |
{kind: "B", d: null};"#
    );
    assert_eq!(
        ComplexEnum::decl(),
r#"export type ComplexEnum = {kind: "A", data: null} |
{kind: "B", data: {
    foo: string,
    bar: number,
}} |
{kind: "W", data: {kind: "A", d: null} |
{kind: "B", d: null}};"#
    );
}
