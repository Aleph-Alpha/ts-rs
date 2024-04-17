#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
struct Foo {
    #[ts(optional, as = "Option<_>")]
    my_optional_bool: bool,
}

#[test]
fn test() {
    assert_eq!(Foo::inline(), "{ my_optional_bool?: boolean, }");
}
