#![allow(dead_code)]

use std::time::Instant;

use ts_rs::TS;

struct Unsupported<T>(T);
struct Unsupported2;

#[derive(TS)]
struct Override {
    a: i32,
    #[ts(type = "0 | 1 | 2")]
    b: i32,
    #[ts(type = "string")]
    x: Instant,
    #[ts(type = "string")]
    y: Unsupported<Unsupported<Unsupported2>>,
    #[ts(type = "string | null")]
    z: Option<Unsupported2>,
}

#[test]
fn test() {
    assert_eq!(
        Override::inline(),
        "{ a: number, b: 0 | 1 | 2, x: string, y: string, z: string | null, }"
    )
}
