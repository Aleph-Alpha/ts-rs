#![cfg(feature = "either-impl")]
#![allow(unused)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "either_impl/")]
struct UsingEither {
    inner: ::either::Either<i32, String>,
}

#[test]
fn inline_either() {
    let cfg = Config::from_env();
    assert_eq!(
        r#"{ "Left": number } | { "Right": string }"#,
        ::either::Either::<i32, String>::inline(&cfg),
    );
}

#[test]
fn using_either() {
    let cfg = Config::from_env();
    assert_eq!(
        r#"{ inner: Either<number, string>, }"#,
        UsingEither::inline(&cfg),
    );
}
