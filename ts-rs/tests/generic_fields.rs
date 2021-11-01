#![allow(dead_code)]
use ts_rs::TS;

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(Vec<i32>);
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
        a: Vec<String>,
    }
    assert_eq!(Struct::inline(), "{ a: Array<string>, }");
}

#[test]
fn named_nested() {
    #[derive(TS)]
    struct Struct {
        a: Vec<Vec<String>>,
    }
    assert_eq!(Struct::inline(), "{ a: Array<Array<string>>, }");
}

#[test]
fn tuple() {
    #[derive(TS)]
    struct Tuple(Vec<i32>, Vec<i32>);
    assert_eq!(Tuple::inline(), "[Array<number>, Array<number>]");
}

#[test]
fn tuple_nested() {
    #[derive(TS)]
    struct Tuple(Vec<Vec<i32>>, Vec<Vec<i32>>);
    assert_eq!(
        Tuple::inline(),
        "[Array<Array<number>>, Array<Array<number>>]"
    );
}
