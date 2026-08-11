#![cfg(feature = "serde_json")]
#![allow(unused)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "serde_json_impl/")]
struct UsingSerdeJson {
    num: serde_json::Number,
    map1: serde_json::Map<String, i32>,
    map2: serde_json::Map<String, UsingSerdeJson>,
    map3: serde_json::Map<String, serde_json::Map<String, i32>>,
    map4: serde_json::Map<String, serde_json::Number>,
    map5: serde_json::Map<String, serde_json::Value>,
    any: serde_json::Value,
}

#[test]
fn using_serde_json() {
    let cfg = Config::from_env();
    assert_eq!(serde_json::Number::inline(&cfg), "number");
    assert_eq!(
        serde_json::Map::<String, i32>::inline(&cfg),
        "{ [key in string]: number }"
    );
    assert_eq!(
        serde_json::Value::decl(&cfg),
        "type JsonValue = number | string | boolean | Array<JsonValue> | { [key in string]: JsonValue } | null;",
    );

    assert_eq!(
        UsingSerdeJson::decl(&cfg),
        "type UsingSerdeJson = { \
            num: number, \
            map1: { [key in string]: number }, \
            map2: { [key in string]: UsingSerdeJson }, \
            map3: { [key in string]: { [key in string]: number } }, \
            map4: { [key in string]: number }, \
            map5: { [key in string]: JsonValue }, \
            any: JsonValue, \
         };"
    )
}

#[derive(TS)]
#[ts(export, export_to = "serde_json_impl/")]
struct InlinedValue {
    #[ts(inline)]
    any: serde_json::Value,
}

#[test]
fn inlined_value() {
    let cfg = Config::from_env();
    assert_eq!(
        InlinedValue::decl(&cfg),
        "type InlinedValue = { \
            any: number | string | boolean | Array<JsonValue> | { [key in string]: JsonValue } | null, \
         };"
    );
}

#[derive(TS)]
#[ts(export, export_to = "serde_json_impl/")]
struct Simple {
    json: serde_json::Value,
}
