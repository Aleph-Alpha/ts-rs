#![cfg(feature = "serde-compat")]

use ts_rs::{Config, TS};

struct Foreign;

#[derive(TS)]
#[ts(export, export_to = "issue_415/")]
struct Issue415 {
    #[ts(optional, type = "Date")]
    a: Option<Foreign>,
}

#[test]
fn issue_415() {
    let cfg = Config::from_env();
    assert_eq!(Issue415::decl(&cfg), "type Issue415 = { a?: Date, };");
}

#[derive(TS)]
#[ts(export, export_to = "issue_415/")]
struct InTuple(i32, #[ts(optional, type = "Date")] Option<Foreign>);

#[test]
fn in_tuple() {
    let cfg = Config::from_env();
    assert_eq!(InTuple::decl(&cfg), "type InTuple = [number, (Date)?];");
}
