#![allow(dead_code)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "arrays/")]
struct Interface {
    a: [i32; 4],
}

#[test]
fn free() {
    let cfg = Config::from_env();
    assert_eq!(<[String; 4]>::inline(&cfg), "[string, string, string, string]")
}

#[test]
fn interface() {
    let cfg = Config::from_env();
    assert_eq!(
        Interface::inline(&cfg),
        "{ a: [number, number, number, number], }"
    )
}

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] [i32; 4]);

    let cfg = Config::from_env();
    assert_eq!(Newtype::inline(&cfg), "[number, number, number, number]")
}
