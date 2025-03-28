#![cfg(feature = "serde-compat")]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "serde_skip_serializing_if/")]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    foo: Option<u8>,
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    bar: bool,
}

#[test]
fn serde_skip_serializing_if() {
    assert_eq!(
        Item::decl(),
        "type Item = { foo?: number, bar?: boolean, };"
    );
}
