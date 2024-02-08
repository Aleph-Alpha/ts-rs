#![allow(clippy::box_collection, clippy::enum_variant_names, dead_code)]
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

macro_rules! declare {
    ($(#[$meta:meta])* $name:ident { $($fident:ident: $t:ty),+ $(,)? }) => {
        $(#[$meta])*
        struct $name {
            $(pub $fident: $t),+
        }
    }
}

declare! {
    #[derive(TS)]
    TypeGroup {
        foo: Vec<Container>,
    }
}

#[test]
fn test() {
    assert_eq!(
        TypeGroup::decl(),
        "type TypeGroup = { foo: Array<Container>, }",
    );

    assert_eq!(
        Generic::<()>::decl(),
        "type Generic<T> = { value: T, values: Array<T>, }"
    );

    assert_eq!(
        GenericAutoBound::<()>::decl(),
        "type GenericAutoBound<T> = { value: T, values: Array<T>, }"
    );

    assert_eq!(
        GenericAutoBound2::<()>::decl(),
        "type GenericAutoBound2<T> = { value: T, values: Array<T>, }"
    );

    assert_eq!(
        Container::decl(),
        "type Container = { foo: Generic<number>, bar: Array<Generic<number>>, baz: Record<string, Generic<string>>, }"
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
        r#"type Generic<A, B, C> = { "A": A } | { "B": [B, B, B] } | { "C": Array<C> } | { "D": Array<Array<Array<A>>> } | { "E": { a: A, b: B, c: C, } } | { "X": Array<number> } | { "Y": number } | { "Z": Array<Array<number>> };"#
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
        "type Struct<T> = { a: T, b: [T, T], c: [T, [T, T]], d: [T, T, T], e: [[T, T], [T, T], [T, T]], f: Array<T>, g: Array<Array<T>>, h: Array<[[T, T], [T, T], [T, T]]>, }"
    )
}

#[test]
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
        t: Generic<Vec<String>>,
    }

    assert_eq!(Generic::<()>::decl(), "type Generic<T> = { t: T, }");
    assert_eq!(
        Container::decl(),
        "type Container = { g: Generic<string>, gi: { t: string, }, t: Array<string>, }"
    );
}

#[test]
#[ignore = "We haven't figured out how to inline generics with bounds yet"]
#[allow(unreachable_code)]
fn inline_with_bounds() {
    todo!("FIX ME: https://github.com/Aleph-Alpha/ts-rs/issues/214");

    #[derive(TS)]
    struct Generic<T: ToString> {
        t: T,
    }

    #[derive(TS)]
    struct Container {
        g: Generic<String>,

        #[ts(inline)]
        gi: Generic<String>,

        #[ts(flatten)]
        t: Generic<u32>,
    }

    assert_eq!(
        Generic::<&'static str>::decl(),
        "type Generic<T> = { t: T, }"
    );
    //                   ^^^^^^^^^^^^ Replace with something else
    assert_eq!(
        Container::decl(),
        "type Container = { g: Generic<string>, gi: { t: string, }, t: number, }" // Actual output: { g: Generic<string>, gi: { t: T, }, t: T, }
    );
}

#[test]
fn inline_with_default() {
    #[derive(TS)]
    struct Generic<T = String> {
        t: T,
    }

    #[derive(TS)]
    struct Container {
        g: Generic<String>,

        #[ts(inline)]
        gi: Generic<String>,

        #[ts(flatten)]
        t: Generic<u32>,
    }

    assert_eq!(
        Generic::<()>::decl(),
        "type Generic<T = string> = { t: T, }"
    );
    assert_eq!(
        Container::decl(),
        "type Container = { g: Generic<string>, gi: { t: string, }, t: number, }"
    );
}

#[test]
fn default() {
    #[derive(TS)]
    struct A<T = String> {
        t: T,
    }
    assert_eq!(A::<()>::decl(), "type A<T = string> = { t: T, }");

    #[derive(TS)]
    struct B<U = Option<A<i32>>> {
        u: U,
    }
    assert_eq!(B::<()>::decl(), "type B<U = A<number> | null> = { u: U, }");
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
    assert_eq!(Y::decl(), "type Y = { a1: A, a2: A<number>, }")
}

#[test]
fn trait_bounds() {
    #[derive(TS)]
    struct A<T: ToString = i32> {
        t: T,
    }
    assert_eq!(A::<i32>::decl(), "type A<T = number> = { t: T, }");

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
        r#"type C<T, K = number> = { "A": { t: T, } } | { "B": T } | "C" | { "D": [T, K] };"#
    );

    #[derive(TS)]
    struct D<T: ToString, const N: usize> {
        t: [T; N],
    }

    let ty = format!(
        "type D<T> = {{ t: [{}], }}",
        "T, ".repeat(41).trim_end_matches(", ")
    );
    assert_eq!(D::<&str, 41>::decl(), ty)
}

#[test]
fn deeply_nested() {
    #[derive(TS)]
    struct T0<T> {
        t0: T,
    }

    #[derive(TS)]
    struct P0<T> {
        p0: T,
    }

    #[derive(TS)]
    struct T1<T> {
        t0: T,
    }

    #[derive(TS)]
    struct P1<T> {
        p0: T,
    }

    #[derive(TS)]
    struct Parent {
        a: T1<T0<u32>>,
        b: T1<P1<T0<P0<u32>>>>,
        c: T1<P1<()>>,
    }
    #[derive(TS)]
    struct GenericParent<T> {
        a_t: T1<T0<T>>,
        b_t: T1<P1<T0<P0<T>>>>,
        c_t: T1<P1<T>>,
        a_null: T1<T0<()>>,
        b_null: T1<P1<T0<P0<()>>>>,
        c_null: T1<P1<()>>,
    }

    assert_eq!(
        Parent::inline(),
        "{ a: T1<T0<number>>, b: T1<P1<T0<P0<number>>>>, c: T1<P1<null>>, }"
    );
    assert_eq!(
        GenericParent::<()>::decl(),
        "type GenericParent<T> = { \
            a_t: T1<T0<T>>, \
            b_t: T1<P1<T0<P0<T>>>>, \
            c_t: T1<P1<T>>, \
            a_null: T1<T0<null>>, \
            b_null: T1<P1<T0<P0<null>>>>, \
            c_null: T1<P1<null>>, \
         }"
    );
}
