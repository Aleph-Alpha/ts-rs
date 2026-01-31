#![allow(dead_code)]
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "list/")]
struct List {
    data: Option<Vec<u32>>,
}

#[test]
fn list() {
    let cfg = Config::from_env();
    assert_eq!(List::decl(&cfg), "type List = { data: Array<number> | null, };");
}
