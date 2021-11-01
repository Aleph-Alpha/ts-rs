#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
struct Optional {
    #[ts(optional)]
    a: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<String>,
}

#[test]
fn test() {
    #[cfg(not(feature = "serde-compat"))]
    assert_eq!(Optional::inline(), "{ a?: number, b: string | null, }");
    #[cfg(feature = "serde-compat")]
    assert_eq!(Optional::inline(), "{ a?: number, b?: string, }")
}
