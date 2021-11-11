#![allow(dead_code)]

use std::{
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

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
struct GenericAutoBound<T> {
    value: T,
    values: Vec<T>,
}

#[derive(TS)]
struct GenericAutoBound2<T>
where
    T: PartialEq,
{
    value: T,
    values: Vec<T>,
}

#[derive(TS)]
struct Container {
    foo: Generic<u32>,
    bar: Box<HashSet<Generic<u32>>>,
    baz: Box<BTreeMap<String, Rc<Generic<String>>>>,
}

#[test]
fn test() {
    assert_eq!(
        Generic::<()>::decl(),
        "interface Generic<T> { value: T, values: Array<T>, }"
    );

    assert_eq!(
        GenericAutoBound::<()>::decl(),
        "interface GenericAutoBound<T> { value: T, values: Array<T>, }"
    );

    assert_eq!(
        GenericAutoBound2::<()>::decl(),
        "interface GenericAutoBound2<T> { value: T, values: Array<T>, }"
    );

    assert_eq!(
        Container::decl(),
        "interface Container { foo: Generic<number>, bar: Array<Generic<number>>, baz: Record<string, Generic<string>>, }"
    );
}

#[test]
fn generic_enum() {
    #[derive(TS)]
    enum Generic<A, B, C> {
        A(A),
        B(B, B, B),
        C(Vec<C>),
        D(Vec<Vec<Vec<A>>>),
        E { a: A, b: B, c: C },
        X(Vec<i32>),
        Y(i32),
        Z(Vec<Vec<i32>>),
    }

    assert_eq!(
        Generic::<(), (), ()>::decl(),
        r#"type Generic<A, B, C> = { A: A } | { B: [B, B, B] } | { C: Array<C> } | { D: Array<Array<Array<A>>> } | { E: { a: A, b: B, c: C, } } | { X: Array<number> } | { Y: number } | { Z: Array<Array<number>> };"#
    )
}

#[test]
fn generic_newtype() {
    #[derive(TS)]
    struct NewType<T>(Vec<Vec<T>>);

    assert_eq!(
        NewType::<()>::decl(),
        r#"type NewType<T> = Array<Array<T>>;"#
    );
}

#[test]
fn generic_tuple() {
    #[derive(TS)]
    struct Tuple<T>(T, Vec<T>, Vec<Vec<T>>);

    assert_eq!(
        Tuple::<()>::decl(),
        r#"type Tuple<T> = [T, Array<T>, Array<Array<T>>];"#
    );
}

#[test]
fn generic_struct() {
    #[derive(TS)]
    struct Struct<T> {
        a: T,
        b: (T, T),
        c: (T, (T, T)),
        d: [T; 3],
        e: [(T, T); 3],
        f: Vec<T>,
        g: Vec<Vec<T>>,
        h: Vec<[(T, T); 3]>,
    }

    assert_eq!(
        Struct::<()>::decl(),
        "interface Struct<T> { a: T, b: [T, T], c: [T, [T, T]], d: Array<T>, e: Array<[T, T]>, f: Array<T>, g: Array<Array<T>>, h: Array<Array<[T, T]>>, }"
    )
}
