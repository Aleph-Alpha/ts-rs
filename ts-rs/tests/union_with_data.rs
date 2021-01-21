#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Bar {
    field: i32
}

#[derive(TS)]
struct Foo {
    bar: Bar
}

#[derive(TS)]
enum SimpleEnum {
    A(String),
    B(i64),
    C,
    D(String, i64),
    E(Foo),
    F {a: i32, b: String}
}

#[test]
fn test_simple_enum() {
    assert_eq!(Foo::decl(), 
r#"export interface Foo {
    bar: Bar,
}"#);
    assert_eq!(
        SimpleEnum::decl(),
r#"export type SimpleEnum = string | number | "C" | [string, number] | {
    bar: Bar,
} | {
    a: number,
    b: string,
};"#
    );
}
