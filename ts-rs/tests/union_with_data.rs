#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Bar {
    field: i32,
}

#[derive(TS)]
struct Foo {
    bar: Bar,
}

#[derive(TS)]
enum SimpleEnum {
    A(String),
    B(i32),
    C,
    D(String, i32),
    E(Foo),
    F { a: i32, b: String },
}

#[test]
fn test_stateful_enum() {
    assert_eq!(Foo::decl(), r#"interface Foo { bar: Bar, }"#);
    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = string | number | "C" | [string, number] | Foo | { a: number, b: string, };"#
    );
}
