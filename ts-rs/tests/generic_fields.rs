#![allow(dead_code, clippy::box_collection)]

use std::borrow::Cow;

use ts_rs::TS;

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(Vec<Cow<'static, i32>>);
    assert_eq!(Newtype::inline(), "Array<number>");
}

#[test]
fn newtype_nested() {
    #[derive(TS)]
    struct Newtype(Vec<Vec<i32>>);
    assert_eq!(Newtype::inline(), "Array<Array<number>>");
}

#[test]
fn alias() {
    type Alias = Vec<String>;
    assert_eq!(Alias::inline(), "Array<string>");
}

#[test]
fn alias_nested() {
    type Alias = Vec<Vec<String>>;
    assert_eq!(Alias::inline(), "Array<Array<string>>");
}

#[test]
fn named() {
    #[derive(TS)]
    struct Struct {
        a: Box<Vec<String>>,
        b: (Vec<String>, Vec<String>),
        c: [Vec<String>; 3],
    }
    assert_eq!(
        Struct::inline(),
        "{ a: Array<string>, b: [Array<string>, Array<string>], c: Array<Array<string>>, }"
    );
}

#[test]
fn named_nested() {
    #[derive(TS)]
    struct Struct {
        a: Vec<Vec<String>>,
        b: (Vec<Vec<String>>, Vec<Vec<String>>),
        c: [Vec<Vec<String>>; 3],
    }
    assert_eq!(Struct::inline(), "{ a: Array<Array<string>>, b: [Array<Array<string>>, Array<Array<string>>], c: Array<Array<Array<string>>>, }");
}

#[test]
fn tuple() {
    #[derive(TS)]
    struct Tuple(Vec<i32>, (Vec<i32>, Vec<i32>), [Vec<i32>; 3]);
    assert_eq!(
        Tuple::inline(),
        "[Array<number>, [Array<number>, Array<number>], Array<Array<number>>]"
    );
}

#[test]
fn tuple_nested() {
    #[derive(TS)]
    struct Tuple(
        Vec<Vec<i32>>,
        (Vec<Vec<i32>>, Vec<Vec<i32>>),
        [Vec<Vec<i32>>; 3],
    );
    assert_eq!(
        Tuple::inline(),
        "[Array<Array<number>>, [Array<Array<number>>, Array<Array<number>>], Array<Array<Array<number>>>]"
    );
}
