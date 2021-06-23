#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Generic<T>
where
    T: TS,
{
    value: T,
}

#[derive(TS)]
struct Container {
    foo: Generic<u32>,
}

#[test]
fn test() {
    assert_eq!(
        Generic::<()>::decl(),
        "\
interface Generic<T> {
    value: T,
}"
    );

    assert_eq!(
        Container::decl(),
        "\
interface Container {
    foo: Generic<number>,
}"
    );
}
