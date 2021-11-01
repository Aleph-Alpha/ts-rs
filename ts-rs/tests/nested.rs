#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct A {
    x1: i32,
    y1: i32,
}

#[derive(TS)]
struct B {
    a1: A,
    #[ts(inline)]
    a2: A,
}

#[derive(TS)]
struct C {
    b1: B,
    #[ts(inline)]
    b2: B,
}

#[test]
fn test_nested() {
    assert_eq!(
        C::inline(),
        "{ b1: B, b2: { a1: A, a2: { x1: number, y1: number, }, }, }"
    );
}
