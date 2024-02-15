#![allow(dead_code)]

use std::collections::HashMap;
use ts_rs::TS;

type TypeAlias = HashMap<String, String>;

#[derive(TS)]
enum Enum {
    A(TypeAlias),
    B(HashMap<String, String>),
}

#[derive(TS)]
struct Struct {
    a: TypeAlias,
    b: HashMap<String, String>
}

#[test]
fn type_alias() {
    assert_eq!(
        Enum::decl(),
        r#"type Enum = { "A": Record<string, string> } | { "B": Record<string, string> };"#
    );

    assert_eq!(
        Struct::decl(),
        "type Struct = { a: Record<string, string>, b: Record<string, string>, }"
    );
}

