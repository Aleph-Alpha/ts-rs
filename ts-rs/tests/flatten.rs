#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "tests-out/flatten/")]
struct A {
    a: i32,
    b: i32,
}

#[derive(TS)]
#[ts(export, export_to = "tests-out/flatten/")]
struct B {
    #[ts(flatten)]
    a: A,
    c: i32,
}

#[derive(TS)]
#[ts(export, export_to = "tests-out/flatten/")]
struct C {
    #[ts(inline)]
    b: B,
    d: i32,
}

#[test]
fn test_def() {
    assert_eq!(
        C::inline(),
        "{ b: { c: number, a: number, b: number, }, d: number, }"
    );
}
