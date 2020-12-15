#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
#[ts(rename_all = "UPPERCASE")]
struct Rename {
    a: i32,
    b: i32,
}

#[test]
fn test() {
    assert_eq!(
        Rename::format(0, true),
        "\
{
    A: number,
    B: number,
}"
    )
}
