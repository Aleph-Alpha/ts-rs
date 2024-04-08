#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, PartialEq, Debug)]
#[ts(export)]
struct Foo {
    #[ts(optional, as = "Option<_>")]
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    my_optional_bool: bool,
}

#[test]
fn test() {
    assert_eq!(Foo::inline(), "{ my_optional_bool?: boolean, }");
}
