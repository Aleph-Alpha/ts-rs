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
pub struct Inner {}

// Create a parent struct that flattens the zero-field struct:
#[derive(TS)]
#[ts(export, export_to = "flatten/")]
pub struct Outer {
    #[ts(flatten)]
    pub inner: Inner,
    pub other_field: String,
}

#[test]
fn test_def() {
    let cfg = Config::from_env();
    assert_eq!(
        C::inline(&cfg),
        "{ b: { c: number, a: number, b: number, } & ({ [key in string]: number }), d: number, }"
    );
    assert_eq!(Outer::inline(&cfg), "{ other_field: string, }");
}
