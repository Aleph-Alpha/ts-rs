#![allow(dead_code)]

use ts_rs::TS;
use serde::Serialize;

#[derive(TS, Serialize)]
#[serde(tag="type")]
struct TaggedType {
    a: i32,
    b: i32,
}

#[test]
#[cfg(feature = "serde-compat")]
fn test() {
    assert_eq!(TaggedType::inline(), "{ type: \"TaggedType\", a: number, b: number, }")
}

#[test]
#[cfg(not(feature = "serde-compat"))]
fn test() {
    assert_eq!(TaggedType::inline(), "{ a: number, b: number, }")
}
