#![allow(unused)]

use ts_rs::TS;

#[test]
fn test_tuple() {
    type Tuple = (String, i32, (i32, i32));
    assert_eq!(
        "[string, number, [number, number]]",
        Tuple::format(0, false)
    );
    assert!(Tuple::decl().is_none());
}

#[test]
fn test_newtype() {
    #[derive(TS)]
    struct NewType(String);

    assert_eq!("export type NewType = string;", NewType::decl().unwrap());
}

#[test]
fn test_tuple_newtype() {
    #[derive(TS)]
    struct TupleNewType(String, i32, (i32, i32));
    assert_eq!(
        "export type TupleNewType = [string, number, [number, number]];",
        TupleNewType::decl().unwrap()
    )
}
