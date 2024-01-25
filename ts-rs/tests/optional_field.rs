#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
struct Optional {
    #[ts(optional)]
    a: Option<i32>,
    b: Option<String>
}

#[test]
fn test() {
    assert_eq!(Optional::inline(), "{ a?: number, b: string | null, }");
}
