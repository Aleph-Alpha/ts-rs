#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Override {
    a: i32,
    #[ts(type = "0 | 1 | 2")]
    b: i32,
}

#[test]
fn test() {
    assert_eq!(
        Override::format(0, true),
        "\
{
    a: number,
    b: 0 | 1 | 2,
}"
    )
}
