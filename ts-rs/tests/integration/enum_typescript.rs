#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "enum_typescript/", use_ts_enum)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
enum A {
    MessageOne,
    MessageTwo,
}

#[test]
fn test_use_typescript_enum() {
    // Let inline funcs still return the normal unions,
    // so as to not break other types that might inline an enum in their definitions
    assert_eq!(
        A::inline_flattened(),
        r#"("messageOne" | "messageTwo")"#,
    );
    assert_eq!(
        A::inline(),
        r#"("messageOne" | "messageTwo")"#,
    );
    // The decl function return the proper typescript enum
    assert_eq!(
        A::decl(),
        r#"enum A { messageOne = "messageOne", messageTwo = "messageTwo" }"#,
    );
    assert_eq!(
        A::decl_concrete(),
        r#"enum A { messageOne = "messageOne", messageTwo = "messageTwo" }"#,
    );
}

#[derive(TS)]
#[ts(export, export_to = "enum_typescript/", use_ts_enum)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "snake_case"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "snake_case"))]
enum B {
    MessageOne,
    MessageTwo,
}

#[test]
fn test_use_typescript_enum_snake_case() {
    assert_eq!(
        B::inline_flattened(),
        r#"("message_one" | "message_two")"#,
    );
    assert_eq!(
        B::inline(),
        r#"("message_one" | "message_two")"#,
    );
    assert_eq!(
        B::decl(),
        r#"enum B { message_one = "message_one", message_two = "message_two" }"#,
    );
    assert_eq!(
        B::decl_concrete(),
        r#"enum B { message_one = "message_one", message_two = "message_two" }"#,
    );
}

#[derive(TS)]
#[ts(export, export_to = "enum_typescript/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
struct Hello {
    hello_there: A,
    #[ts(inline)]
    good_night: B,
}

#[test]
fn test_use_typescript_enum_within_struct() {
    assert_eq!(
        Hello::inline_flattened(),
        r#"{ helloThere: A, goodNight: ("message_one" | "message_two"), }"#,
    );
    assert_eq!(
        Hello::inline(),
        r#"{ helloThere: A, goodNight: ("message_one" | "message_two"), }"#,
    );
    assert_eq!(
        Hello::decl(),
        r#"type Hello = { helloThere: A, goodNight: ("message_one" | "message_two"), };"#,
    );
    assert_eq!(
        Hello::decl_concrete(),
        r#"type Hello = { helloThere: A, goodNight: ("message_one" | "message_two"), };"#,
    );
}

#[derive(TS)]
#[ts(export, export_to = "enum_typescript/", use_ts_enum)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "PascalCase"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "PascalCase"))]
enum C {
    MessageOne,
    #[serde(rename = "boo_yah")]
    MessageTwo,
}

#[test]
fn test_use_typescript_enum_with_rename_variant() {
    // Let inline funcs still return the normal unions,
    // so as to not break other types that might inline an enum in their definitions
    assert_eq!(
        C::inline_flattened(),
        r#"("MessageOne" | "boo_yah")"#,
    );
    assert_eq!(
        C::inline(),
        r#"("MessageOne" | "boo_yah")"#,
    );
    // The decl function return the proper typescript enum
    assert_eq!(
        C::decl(),
        r#"enum C { MessageOne = "MessageOne", boo_yah = "boo_yah" }"#,
    );
    assert_eq!(
        C::decl_concrete(),
        r#"enum C { MessageOne = "MessageOne", boo_yah = "boo_yah" }"#,
    );
}
