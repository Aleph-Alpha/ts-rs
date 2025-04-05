#![cfg(feature = "serde-compat")]

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "serde_skip_serializing/")]
pub struct Item {
    // Serialization produces:  `number |  undefined  | null(1)`
    //                          (1): We'd know `null` is never produced if we checked the predicate
    // Deserialization accepts: `number | null`
    //                          (2): `undefined` is also accepted, but only with serde_json
    //
    // There is no type we can choose which accepts every possible value in both directions.
    // Therefore, we stick with the default, ignoring the annotations: `a: number | null`.
    //
    // When TS receives a value from Rust, users might get `undefined`, causing a runtime error.
    // When TS sends a value to Rust, the type is guaranteed to be correct.
    // => Possible runtime error in TS
    //
    // If we instead generated `a?: number`:
    // When TS receives a value from Rust, a runtime error may occur if the value is `null` (1)
    // When TS sends a value to Rust, a runtime error may occur if the value is `undefined` (2)
    // => Possible runtime errors in TS and Rust
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<u8>,

    // Serialization produces:  `boolean | undefined`
    // Deserialization accepts: `boolean | undefined`
    // Most general binding:    `b?: boolean`
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    b: bool,

    // Serialization produces:  `number | undefined | null(1)`
    // Deserialization accepts: `number | undefined | null`
    // Most general binding:    `c?: number | null`
    //
    // (1): If we could ensure that the predicate always skips for `None`, then we'd know for sure
    //      that serialization never produces `null`.
    //      Then, we'd have the choice between `c?: number` and `c?: number | null`.
    //      The first would incorrectly prevent users from deserializing `null`, and
    //      the second wouldn't tell users that `null` is never serialized
    #[serde(skip_serializing_if = "Option::is_none", default)]
    c: Option<i32>,

    // Serialization produces:  `         undefined       `
    // Deserialization accepts: `number | undefined | null`
    // Most general binding:    `d?: number | null`
    #[serde(skip_serializing, default)]
    d: Option<i32>,
}

#[test]
fn serde_skip_serializing_if() {
    assert_eq!(
        Item::decl(),
        "type Item = { a: number | null, b?: boolean, c?: number | null, d?: number | null, };"
    );
}
