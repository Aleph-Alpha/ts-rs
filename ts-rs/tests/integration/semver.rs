#![allow(dead_code)]
#![cfg(feature = "semver-impl")]

use semver::Version;
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "semver/")]
struct Semver {
    version: Version,
}

#[test]
fn semver() {
    let cfg = Config::from_env();
    assert_eq!(Semver::decl(&cfg), "type Semver = { version: string, };")
}
