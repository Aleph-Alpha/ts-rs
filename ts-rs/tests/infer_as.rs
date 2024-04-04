#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, PartialEq, Debug)]
#[ts(export)]
struct Foo {
    #[ts(optional, as = "Option<_>")]
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    my_optional_bool: bool,

    #[ts(as = "std::collections::HashMap<String, _>")]
    a: i32,
}

#[test]
fn test() {
    let falsy = Foo {
        my_optional_bool: false,
        a: 0,
    };
    let truthy = Foo {
        my_optional_bool: true,
        a: 0,
    };

    // Type definition
    assert_eq!(Foo::inline(), "{ my_optional_bool?: boolean, }");

    // Serializing
    assert_eq!(serde_json::to_string(&falsy).unwrap(), "{}"); // `false` is not serialized
    assert_eq!(
        serde_json::to_string(&truthy).unwrap(),
        r#"{"my_optional_bool":true}"#
    );

    // Deserializing
    assert_eq!(
        serde_json::from_str::<Foo>(r#"{"my_optional_bool":true}"#).unwrap(),
        truthy
    );
    assert_eq!(
        serde_json::from_str::<Foo>(r#"{"my_optional_bool":false}"#).unwrap(),
        falsy
    );
    assert_eq!(serde_json::from_str::<Foo>("{}").unwrap(), falsy); // `undefined` is deserialized into `false`
}
