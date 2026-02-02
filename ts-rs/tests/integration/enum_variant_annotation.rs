#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "enum_variant_anotation/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "SCREAMING_SNAKE_CASE"))]
enum A {
    MessageOne {
        sender_id: String,
        number_of_snakes: u64,
    },
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    MessageTwo {
        sender_id: String,
        number_of_camels: u64,
    },
}

#[test]
fn test_enum_variant_rename_all() {
    let cfg = Config::from_env();
    assert_eq!(
        A::inline(&cfg),
        r#"{ "MESSAGE_ONE": { sender_id: string, number_of_snakes: bigint, } } | { "MESSAGE_TWO": { senderId: string, numberOfCamels: bigint, } }"#,
    );
}

#[derive(TS)]
#[ts(export, export_to = "enum_variant_anotation/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
enum B {
    #[cfg_attr(feature = "serde-compat", serde(rename = "SnakeMessage"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename = "SnakeMessage"))]
    MessageOne {
        sender_id: String,
        number_of_snakes: u64,
    },
    #[cfg_attr(feature = "serde-compat", serde(rename = "CamelMessage"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename = "CamelMessage"))]
    MessageTwo {
        sender_id: String,
        number_of_camels: u64,
    },
}

#[test]
fn test_enum_variant_rename() {
    let cfg = Config::from_env();
    assert_eq!(
        B::inline(&cfg),
        r#"{ "SnakeMessage": { sender_id: string, number_of_snakes: bigint, } } | { "CamelMessage": { sender_id: string, number_of_camels: bigint, } }"#,
    );
}

#[derive(TS)]
#[ts(export, export_to = "enum_variant_anotation/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "kind"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "kind"))]
pub enum C {
    #[cfg_attr(feature = "serde-compat", serde(rename = "SQUARE_THING"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename = "SQUARE_THING"))]
    SquareThing {
        name: String,
        // ...
    },
}

#[test]
fn test_enum_variant_with_tag() {
    let cfg = Config::from_env();
    assert_eq!(C::inline(&cfg), r#"{ "kind": "SQUARE_THING", name: string, }"#);
}

#[cfg(feature = "serde-compat")]
#[test]
fn test_tag_and_content_quoted() {
    #[derive(Serialize, TS)]
    #[serde(tag = "kebab-cased-tag", content = "whitespace in content")]
    enum E {
        V { f: String },
    }
    let cfg = Config::from_env();
    assert_eq!(
        E::inline(&cfg),
        r#"{ "kebab-cased-tag": "V", "whitespace in content": { f: string, } }"#
    )
}

#[cfg(feature = "serde-compat")]
#[test]
fn test_variant_quoted() {
    #[derive(Serialize, TS)]
    #[serde(rename_all = "kebab-case")]
    enum E {
        VariantName { f: String },
    }
    let cfg = Config::from_env();
    assert_eq!(E::inline(&cfg), r#"{ "variant-name": { f: string, } }"#)
}

#[derive(TS)]
#[ts(export, export_to = "enum_variant_anotation/")]
enum D {
    Foo {},
}

#[derive(TS)]
#[ts(export, export_to = "enum_variant_anotation/", tag = "type")]
enum E {
    Foo {},
    Bar {},
    Biz { x: i32 },
}

#[test]
fn test_empty_struct_variant_with_tag() {
    let cfg = Config::from_env();
    assert_eq!(
        E::inline(&cfg),
        r#"{ "type": "Foo", } | { "type": "Bar", } | { "type": "Biz", x: number, }"#
    )
}
