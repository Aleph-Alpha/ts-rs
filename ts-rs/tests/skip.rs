#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Skip {
    a: i32,
    b: i32,
    #[ts(skip)]
    c: String,
}

#[test]
fn test_def() {
    assert_eq!(
        Skip::format(0, true),
        "\
{
    a: number,
    b: number,
}"
    );
}
