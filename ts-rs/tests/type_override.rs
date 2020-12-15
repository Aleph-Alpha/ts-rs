#![allow(dead_code)]

use ts_rs::TS;
use std::time::Instant;

#[derive(TS)]
struct Override {
    a: i32,
    #[ts(type = "0 | 1 | 2")]
    b: i32,
    #[ts(type = "string")]
    x: Instant
}

#[test]
fn test() {
    assert_eq!(
        Override::format(0, true),
        "\
{
    a: number,
    b: 0 | 1 | 2,
    x: string,
}"
    )
}
