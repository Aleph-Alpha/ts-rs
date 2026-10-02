#![cfg(feature = "nonempty-impl")]
#![allow(unused)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "nonempty_impl/")]
struct UsingNonempty {
    inner: ::nonempty::NonEmpty<i32>,
}

#[test]
fn using_nonempty() {
    let cfg = Config::from_env();
    assert_eq!(UsingNonempty::inline(&cfg), r#"{ inner: Array<number>, }"#);
}
