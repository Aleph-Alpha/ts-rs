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
    assert_eq!(Skip::inline(), "{ a: number, b: number, }");
}
