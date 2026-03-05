#![allow(dead_code)]
#![cfg(feature = "indexmap-impl")]

use indexmap::{IndexMap, IndexSet};
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "indexmap/")]
struct Indexes {
    map: IndexMap<String, String>,
    set: IndexSet<String>,
}

#[test]
fn indexmap() {
    let cfg = Config::from_env();
    assert_eq!(
        Indexes::decl(&cfg),
        "type Indexes = { map: { [key in string]: string }, set: Array<string>, };"
    )
}
