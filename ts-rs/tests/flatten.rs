#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct A {
    a: i32,
    b: i32,
}

#[derive(TS)]
struct B {
    #[ts(flatten)]
    a: A,
    c: i32,
}

#[derive(TS)]
struct C {
    #[ts(inline)]
    b: B,
    d: i32,
}

#[test]
fn test_def() {
    assert_eq!(
        C::inline(),
        "{ b: { a: number, b: number, c: number, }, d: number, }"
    );
}
