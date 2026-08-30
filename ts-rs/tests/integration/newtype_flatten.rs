#![allow(dead_code)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "newtype_flatten/")]
struct Inner {
    foo: String,
    bar: i32,
}

#[derive(TS)]
#[ts(export, export_to = "newtype_flatten/")]
struct NewtypeWrapper(Inner);

#[derive(TS)]
#[ts(export, export_to = "newtype_flatten/")]
struct Outer {
    name: String,
    #[ts(flatten)]
    inner: NewtypeWrapper,
}

#[test]
fn test_newtype_flatten() {
    let cfg = Config::from_env();
    assert_eq!(Inner::inline(&cfg), "{ foo: string, bar: number, }");
    assert_eq!(NewtypeWrapper::inline(&cfg), "Inner");
    assert_eq!(
        Outer::inline(&cfg),
        "{ name: string, foo: string, bar: number, }"
    );
}

#[derive(TS)]
#[ts(export, export_to = "newtype_flatten/")]
struct InlinedNewtypeWrapper(#[ts(inline)] Inner);

#[derive(TS)]
#[ts(export, export_to = "newtype_flatten/")]
struct OuterInlined {
    name: String,
    #[ts(flatten)]
    inner: InlinedNewtypeWrapper,
}

#[test]
fn test_newtype_flatten_inlined() {
    let cfg = Config::from_env();
    assert_eq!(
        InlinedNewtypeWrapper::inline(&cfg),
        "{ foo: string, bar: number, }"
    );
    assert_eq!(
        OuterInlined::inline(&cfg),
        "{ name: string, foo: string, bar: number, }"
    );
}
