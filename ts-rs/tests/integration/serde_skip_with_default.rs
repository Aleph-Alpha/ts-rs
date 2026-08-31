#![cfg(feature = "serde-compat")]
#![allow(dead_code)]

// from issue #107. This does now no longer generate a warning.

use serde::{Deserialize, Serialize};
use ts_rs::{Config, TS};

fn default_http_version() -> String {
    "2".to_owned()
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "serde_skip_with_default/")]
pub struct Foobar {
    // #[ts(skip)]
    #[serde(skip, default = "default_http_version")]
    pub http_version: String,
    pub something_else: i32,
}

#[test]
fn serde_skip_with_default() {
    let cfg = Config::from_env();
    assert_eq!(Foobar::decl(&cfg), "type Foobar = { something_else: number, };");
}
