#![allow(unused)]

use std::collections::HashMap;

use ts_rs::{Config, TS};

type TypeAlias = HashMap<String, String>;

#[derive(TS)]
#[ts(export, export_to = "issue_70/")]
enum Enum {
    A(TypeAlias),
    B(HashMap<String, String>),
}

#[derive(TS)]
#[ts(export, export_to = "issue_70/")]
struct Struct {
    a: TypeAlias,
    b: HashMap<String, String>,
}

#[test]
fn issue_70() {
    let cfg = Config::from_env();
    assert_eq!(
        Enum::decl(&cfg),
        "type Enum = { \"A\": { [key in string]: string } } | { \"B\": { [key in string]: string } };"
    );
    assert_eq!(
        Struct::decl(&cfg),
        "type Struct = { a: { [key in string]: string }, b: { [key in string]: string }, };"
    );
}

#[derive(TS)]
#[ts(export, export_to = "issue_70/")]
struct GenericType<T, U> {
    foo: T,
    bar: U,
}

type GenericAlias<A = String, B = String> = GenericType<(A, String), Vec<(B, i32)>>;

#[derive(TS)]
#[ts(export, export_to = "issue_70/")]
struct Container {
    a: GenericAlias<Vec<i32>, Vec<String>>,
    b: GenericAlias,
}

#[derive(TS)]
#[ts(export, export_to = "issue_70/")]
struct GenericContainer<A, B = i32> {
    a: GenericAlias,
    b: GenericAlias<A, B>,
    c: GenericAlias<A, GenericAlias<A, B>>,
}

#[test]
fn generic() {
    let cfg = Config::from_env();
    assert_eq!(
        Container::decl(&cfg),
        "type Container = { \
            a: GenericType<[Array<number>, string], Array<[Array<string>, number]>>, \
            b: GenericType<[string, string], Array<[string, number]>>, \
        };"
    );

    assert_eq!(
        GenericContainer::<(), ()>::decl(&cfg),
        "type GenericContainer<A, B = number> = { \
            a: GenericType<[string, string], Array<[string, number]>>, \
            b: GenericType<[A, string], Array<[B, number]>>, \
            c: GenericType<[A, string], Array<[GenericType<[A, string], Array<[B, number]>>, number]>>, \
        };"
    );
}
