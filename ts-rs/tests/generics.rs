#![allow(dead_code)]

use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
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

#[test]
#[ignore]
// https://github.com/Aleph-Alpha/ts-rs/issues/56 TODO
fn inline() {
    #[derive(TS)]
    struct Generic<T> {
        t: T,
    }

    #[derive(TS)]
    struct Container {
        g: Generic<String>,
        #[ts(inline)]
        gi: Generic<String>,
        #[ts(flatten)]
        t: Generic<String>,
    }

    assert_eq!(Generic::<()>::decl(), "interface Generic<T> { t: T, }");
    assert_eq!(
        Container::decl(),
        "interface Container { g: Generic<string>, gi: { t: string }, t: string, }"
    );
}

#[test]
fn default() {
    #[derive(TS)]
    struct A<T = String> {
        t: T,
    }
    assert_eq!(A::<()>::decl(), "interface A<T = string> { t: T, }");

    #[derive(TS)]
    struct B<U = Option<A<i32>>> {
        u: U,
    }
    assert_eq!(
        B::<()>::decl(),
        "interface B<U = A<number> | null> { u: U, }"
    );
    assert!(B::<()>::dependencies().iter().any(|dep| dep.ts_name == "A"));

    #[derive(TS)]
    struct Y {
        a1: A,
        a2: A<i32>,
        // https://github.com/Aleph-Alpha/ts-rs/issues/56
        // TODO: fixme
        // #[ts(inline)]
        // xi: X,
        // #[ts(inline)]
        // xi2: X<i32>
    }
    assert_eq!(Y::decl(), "interface Y { a1: A, a2: A<number>, }")
}

#[test]
fn trait_bounds() {
    #[derive(TS)]
    struct A<T: ToString = i32> {
        t: T,
    }
    assert_eq!(A::<i32>::decl(), "interface A<T = number> { t: T, }");

    #[derive(TS)]
    struct B<T: ToString + Debug + Clone + 'static>(T);
    assert_eq!(B::<&'static str>::decl(), "type B<T> = T;");

    #[derive(TS)]
    enum C<T: Copy + Clone + PartialEq, K: Copy + PartialOrd = i32> {
        A { t: T },
        B(T),
        C,
        D(T, K),
    }
    assert_eq!(
        C::<&'static str, i32>::decl(),
        "type C<T, K = number> = { A: { t: T, } } | { B: T } | \"C\" | { D: [T, K] };"
    );

    #[derive(TS)]
    struct D<T: ToString, const N: usize> {
        t: [T; N],
    }

    assert_eq!(D::<&str, 41>::decl(), "interface D<T> { t: Array<T>, }")
}

#[test]
fn nonstatic_lifetimes() {
    #[derive(TS)]
    struct A<'a> {
        t: &'a str,
    }
    assert_eq!(A::decl(), "interface A<> { t: string, }");
}

#[test]
fn nonstatic_lifetimes_with_child() {
    #[derive(TS)]
    struct A<'a> {
        t: &'a str,
    }

    #[derive(TS)]
    struct B<'a> {
        t: A<'a>,
    }
    assert_eq!(B::decl(), "interface B<> { t: A, }");
}
