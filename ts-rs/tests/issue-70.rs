#![allow(unused)]

use std::collections::HashMap;

use ts_rs::TS;

#[test]
fn issue_70() {
    type TypeAlias = HashMap<String, String>;

    #[derive(TS)]
    enum Enum {
        A(TypeAlias),
        B(HashMap<String, String>),
    }

    #[derive(TS)]
    struct Struct {
        a: TypeAlias,
        b: HashMap<String, String>,
    }

    assert_eq!(
        Enum::decl(),
        "type Enum = { \"A\": Record<string, string> } | { \"B\": Record<string, string> };"
    );
    assert_eq!(
        Struct::decl(),
        "type Struct = { a: Record<string, string>, b: Record<string, string>, };"
    );
}

#[test]
fn generic() {
    type GenericAlias<A = String, B = String> = HashMap<(A, String), Vec<(B, i32)>>;

    #[derive(TS)]
    struct Container {
        a: GenericAlias<Vec<i32>, Vec<String>>,
        b: GenericAlias,
    }
    assert_eq!(
        Container::decl(),
        "type Container = { \
            a: Record<[Array<number>, string], Array<[Array<string>, number]>>, \
            b: Record<[string, string], Array<[string, number]>>, \
        };"
    );

    #[derive(TS)]
    struct GenericContainer<A, B = i32> {
        a: GenericAlias,
        b: GenericAlias<A, B>,
        c: GenericAlias<A, GenericAlias<A, B>>,
    }
    assert_eq!(
        GenericContainer::<(), ()>::decl(),
        "type GenericContainer<A, B = number> = { \
            a: Record<[string, string], Array<[string, number]>>, \
            b: Record<[A, string], Array<[B, number]>>, \
            c: Record<[A, string], Array<[Record<[A, string], Array<[B, number]>>, number]>>, \
        };"
    );
}
