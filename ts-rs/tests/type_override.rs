#![allow(dead_code)]

use std::time::Instant;

use ts_rs::TS;

#[derive(TS)]
struct Override {
    a: i32,
    #[ts(type = "0 | 1 | 2")]
    b: i32,
    #[ts(type = "string")]
    x: Instant,
}

#[test]
fn test() {
    assert_eq!(
        Override::inline(),
        "{ a: number, b: 0 | 1 | 2, x: string, }"
    )
}
