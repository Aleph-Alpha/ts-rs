#![allow(unused)]

use ts_rs::{Config, TS};

#[test]
fn free() {
    let cfg = Config::from_env();
    assert_eq!(<[String]>::inline(&cfg), "Array<string>")
}

#[derive(TS)]
#[ts(export, export_to = "slices/")]
struct Interface {
    #[allow(dead_code)]
    a: [i32],
}

#[test]
fn interface() {
    let cfg = Config::from_env();
    assert_eq!(Interface::inline(&cfg), "{ a: Array<number>, }")
}

#[derive(TS)]
#[ts(export, export_to = "slices/")]
struct InterfaceRef<'a> {
    #[allow(dead_code)]
    a: &'a [&'a str],
}

#[test]
fn slice_ref() {
    let cfg = Config::from_env();
    assert_eq!(InterfaceRef::inline(&cfg), "{ a: Array<string>, }")
}

#[derive(TS)]
#[ts(export, export_to = "slices/")]
struct Newtype(#[allow(dead_code)] [i32]);

#[test]
fn newtype() {
    let cfg = Config::from_env();
    assert_eq!(Newtype::inline(&cfg), "Array<number>")
}

// Since slices usually need to be wrapped in a `Box` or other container,
// these tests should to check for that

#[test]
fn boxed_free() {
    let cfg = Config::from_env();
    assert_eq!(<Box<[String]>>::inline(&cfg), "Array<string>")
}

#[derive(TS)]
#[ts(export, export_to = "slices/")]
struct InterfaceBoxed {
    #[allow(dead_code)]
    a: Box<[i32]>,
}

#[test]
fn boxed_interface() {
    let cfg = Config::from_env();
    assert_eq!(InterfaceBoxed::inline(&cfg), "{ a: Array<number>, }")
}

#[derive(TS)]
#[ts(export, export_to = "slices/")]
struct NewtypeBoxed(#[allow(dead_code)] Box<[i32]>);

#[test]
fn boxed_newtype() {
    let cfg = Config::from_env();
    assert_eq!(NewtypeBoxed::inline(&cfg), "Array<number>")
}

#[derive(TS)]
#[ts(export, export_to = "slices/nested/")]
struct InnerMost;

#[derive(TS)]
#[ts(export, export_to = "slices/nested/")]
struct Nested<'a>(&'a [InnerMost]);
