#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS, serde::Serialize)]
#[ts(export)]
struct Foo {
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    #[ts(optional, as = "Option<_>")]
    my_optional_bool: bool,
}

#[test]
fn test() {
    assert_eq!(Foo::inline(), "{ my_optional_bool?: boolean, }");
}
