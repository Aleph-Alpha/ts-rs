use std::collections::HashMap;

#[cfg(feature = "serde-json-impl")]
use serde_json::Value as JsonValue;
use ts_rs::TS;

#[derive(TS)]
#[ts(as = "T")]
pub enum UntaggedEnum<T: TS> {
    Left(T),
    Right(T),
}

#[test]
pub fn top_level_type_as_enum() {
    assert_eq!(UntaggedEnum::<String>::inline(), r#"string"#)
}

#[derive(TS)]
#[ts(as = "T")]
pub struct Wrapper<T: TS>(T);

#[test]
pub fn top_level_type_as_struct() {
    assert_eq!(Wrapper::<String>::inline(), r#"string"#)
}

#[cfg(feature = "serde-json-impl")]
#[derive(TS)]
#[ts(
    export,
    export_to = "top_level_type_as/",
    as = "HashMap::<String, JsonValue>"
)]
pub struct JsonMap(JsonValue);

#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/")]
pub struct Foo {
    x: i32,
}

#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/")]
pub struct Bar {
    foo: Foo,
}

#[derive(TS)]
#[ts(
    export,
    export_to = "top_level_type_as/",
    as = "HashMap::<String, Bar>"
)]
pub struct Biz(String);

// -- test that TS::IS_ENUM is preserved correctly --

pub struct Unsupported;

#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/")]
pub enum NormalEnum {
    A,
    B,
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/")]
pub struct NormalStruct {
    x: u32,
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/", as = "String")]
pub enum EnumAsString {
    A(Unsupported),
    B(Unsupported),
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/", as = "NormalEnum")]
pub enum EnumAsEnum {
    A(Unsupported),
    B(Unsupported),
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/", as = "NormalStruct")]
pub enum EnumAsStruct {
    A(Unsupported),
    B(Unsupported),
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/", as = "String")]
pub struct StructAsString {
    x: Unsupported,
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/", as = "NormalEnum")]
pub struct StructAsEnum {
    x: Unsupported,
}
#[derive(TS)]
#[ts(export, export_to = "top_level_type_as/", as = "NormalStruct")]
pub struct StructAsStruct {
    x: Unsupported,
}

#[test]
fn preserves_is_enum() {
    assert!(NormalEnum::IS_ENUM);
    assert!(!NormalStruct::IS_ENUM);

    assert_eq!(EnumAsString::inline(), String::inline());
    assert!(!EnumAsString::IS_ENUM);
    assert_eq!(EnumAsEnum::inline(), NormalEnum::inline());
    assert!(EnumAsEnum::IS_ENUM);
    assert_eq!(EnumAsStruct::inline(), NormalStruct::inline());
    assert!(!EnumAsStruct::IS_ENUM);

    assert_eq!(StructAsString::inline(), String::inline());
    assert!(!StructAsString::IS_ENUM);
    assert_eq!(StructAsEnum::inline(), NormalEnum::inline());
    assert!(StructAsEnum::IS_ENUM);
    assert_eq!(StructAsStruct::inline(), NormalStruct::inline());
    assert!(!StructAsStruct::IS_ENUM);
}
