#![allow(dead_code)]

use std::collections::HashMap;
use ts_rs::TS;

#[derive(TS)]
struct Generic<T>
where
    T: TS,
{
    value: T,
    values: Vec<T>,
}

#[derive(TS)]
struct Container {
    foo: Generic<u32>,
    bar: Vec<Generic<u32>>,
    baz: HashMap<String, Generic<String>>,
}

#[test]
fn test() {
    assert_eq!(
        Generic::<()>::decl(),
        "\
interface Generic<T> {
    value: T,
    values: Array<T>,
}"
    );

    assert_eq!(
        Container::decl(),
        "\
interface Container {
    foo: Generic<number>,
    bar: Array<Generic<number>>,
    baz: Record<string, Generic<string>>,
}"
    );
}
