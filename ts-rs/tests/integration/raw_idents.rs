#![allow(non_camel_case_types, dead_code)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "raw_idents/")]
struct r#struct {
    r#type: i32,
    r#use: i32,
    r#struct: i32,
    r#let: i32,
    r#enum: i32,
}

#[test]
fn raw_idents() {
    let cfg = Config::from_env();
    let out = <r#struct as TS>::decl(&cfg);
    assert_eq!(
        out,
        "type struct = { type: number, use: number, struct: number, let: number, enum: number, };"
    );
}
