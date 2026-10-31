#![allow(dead_code)]

use std::collections::HashMap;

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "flatten/")]
struct A {
    a: i32,
    b: i32,
    #[ts(flatten)]
    c: HashMap<String, i32>,
}

#[derive(TS)]
#[ts(export, export_to = "flatten/")]
struct B {
    #[ts(flatten)]
    a: A,
    c: i32,
}

#[derive(TS)]
#[ts(export, export_to = "flatten/")]
struct C {
    #[ts(inline)]
    b: B,
    d: i32,
}

#[derive(TS)]
#[ts(export, export_to = "flatten/")]
struct Empty {}

#[derive(TS)]
#[ts(export, export_to = "flatten/")]
struct FlattenEmpty {
    #[ts(flatten)]
    empty: Empty,
    other_field: String,
}

#[derive(TS)]
#[ts(export, export_to = "flatten/")]
struct FlattenOnlyEmpty {
    #[ts(flatten)]
    empty: Empty,
}

#[test]
fn test_def() {
    let cfg = Config::from_env();
    assert_eq!(
        C::inline(&cfg),
        "{ b: { c: number, a: number, b: number, } & ({ [key in string]: number }), d: number, }"
    );
    assert_eq!(
        FlattenEmpty::inline(&cfg),
        "{ other_field: string, }"
    );
    assert_eq!(
        FlattenOnlyEmpty::inline(&cfg),
        "{  }"
    );
}
