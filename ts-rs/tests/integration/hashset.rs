#![allow(dead_code)]

use std::collections::{BTreeSet, HashSet};

use ts_rs::{Config, TS};

#[derive(TS, Eq, PartialEq, Hash)]
#[ts(export, export_to = "hashset/")]
struct CustomValue;

#[derive(TS)]
#[ts(export, export_to = "hashset/")]
struct HashSetWithCustomType {
    set: HashSet<CustomValue>,
}

#[derive(TS)]
#[ts(export, export_to = "hashset/")]
struct BTreeSetWithCustomType {
    set: BTreeSet<CustomValue>,
}

#[test]
fn with_custom_types() {
    let cfg = Config::from_env();
    assert_eq!(
        HashSetWithCustomType::inline(&cfg),
        BTreeSetWithCustomType::inline(&cfg)
    );
    assert_eq!(
        HashSetWithCustomType::decl(&cfg),
        "type HashSetWithCustomType = { set: Array<CustomValue>, };"
    );
}
