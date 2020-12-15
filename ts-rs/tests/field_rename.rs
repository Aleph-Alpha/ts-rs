#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Rename {
    a: i32,
    #[ts(rename = "bb")]
    b: i32,
}

#[test]
fn test() {
    assert_eq!(
        Rename::format(0, true),
        "\
{
    a: number,
    bb: number,
}"
    )
}
