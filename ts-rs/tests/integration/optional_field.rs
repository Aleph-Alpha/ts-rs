#![allow(dead_code)]

use serde::Serialize;
use ts_rs::{Config, TS};

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct OptionalInStruct {
    #[ts(optional)]
    a: Option<i32>,
    #[ts(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[test]
fn in_struct() {
    let a = "a?: number";
    let b = "b?: number | null";
    let c = "c: number | null";
    let cfg = Config::from_env();
    assert_eq!(OptionalInStruct::inline(&cfg), format!("{{ {a}, {b}, {c}, }}"));
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct GenericOptionalStruct<T> {
    #[ts(optional)]
    a: Option<T>,
}

#[test]
fn in_generic_struct() {
    let cfg = Config::from_env();
    assert_eq!(
        GenericOptionalStruct::<()>::decl(&cfg),
        "type GenericOptionalStruct<T> = { a?: T, };"
    )
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
enum OptionalInEnum {
    A {
        #[ts(optional)]
        a: Option<i32>,
    },
    B {
        b: Option<String>,
    },
}

#[test]
fn in_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalInEnum::inline(&cfg),
        r#"{ "A": { a?: number, } } | { "B": { b: string | null, } }"#
    );
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct OptionalFlatten {
    #[ts(optional)]
    a: Option<i32>,
    #[ts(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct Flatten {
    #[ts(flatten)]
    x: OptionalFlatten,
}

#[test]
fn flatten() {
    let cfg = Config::from_env();
    assert_eq!(Flatten::inline(&cfg), OptionalFlatten::inline(&cfg));
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct OptionalInline {
    #[ts(optional)]
    a: Option<i32>,
    #[ts(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct Inline {
    #[ts(inline)]
    x: OptionalInline,
}

#[test]
fn inline() {
    let a = "a?: number";
    let b = "b?: number | null";
    let c = "c: number | null";
    let cfg = Config::from_env();
    assert_eq!(Inline::inline(&cfg), format!("{{ x: {{ {a}, {b}, {c}, }}, }}"));
}

type Foo = Option<i32>;
type Bar<T> = Option<T>;

#[derive(TS)]
#[ts(export, export_to = "optional_field/", optional_fields)]
struct OptionalStruct {
    a: Option<i32>,
    b: Option<i32>,

    #[ts(optional = nullable)]
    c: Option<i32>,

    d: i32,

    e: Foo,
    f: Bar<i32>,

    #[ts(type = "string")]
    g: Option<i32>,

    #[ts(as = "String")]
    h: Option<i32>,
}

#[test]
fn struct_optional() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalStruct::inline(&cfg),
        format!(
            "{{ a?: number, b?: number, c?: number | null, d: number, e?: number, f?: number, g: string, h: string, }}"
        )
    )
}

#[derive(TS)]
#[ts(export, export_to = "optional_field/", optional_fields = nullable)]
struct NullableStruct {
    a: Option<i32>,
    b: Option<i32>,

    #[ts(optional = nullable)]
    c: Option<i32>,

    d: i32,

    e: Foo,
    f: Bar<i32>,

    #[ts(type = "string")]
    g: Option<i32>,

    #[ts(as = "String")]
    h: Option<i32>,

    // not nullable
    #[ts(optional)]
    i: Option<i32>,

    // not optional
    #[ts(optional = false)]
    j: Option<i32>,
}

#[test]
fn struct_nullable() {
    let cfg = Config::from_env();
    assert_eq!(
        NullableStruct::inline(&cfg),
        format!(
            "{{ a?: number | null, b?: number | null, c?: number | null, d: number, e?: number | null, f?: number | null, g: string, h: string, i?: number, j: number | null, }}"
        )
    )
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
struct OptionalInTuple(
    Option<i32>,
    #[ts(optional)] Option<i32>,
    #[ts(optional = nullable)] Option<i32>,
);

#[test]
fn in_tuple() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalInTuple::inline(&cfg),
        format!("[number | null, (number)?, (number | null)?]")
    );
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
#[ts(optional_fields)]
struct OptionalTuple(
    i32,
    #[ts(type = "string")] Option<i32>,
    #[ts(as = "String")] Option<i32>,
    Option<i32>,
    #[ts(optional)] Option<i32>,
    #[ts(optional = nullable)] Option<i32>,
);

#[test]
fn tuple_optional() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalTuple::inline(&cfg),
        "[number, string, string, (number)?, (number)?, (number | null)?]"
    );
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/")]
#[ts(optional_fields = nullable)]
struct NullableTuple(
    i32,
    #[ts(type = "string")] Option<i32>,
    #[ts(as = "String")] Option<i32>,
    Option<i32>,
    #[ts(optional)] Option<i32>,
    #[ts(optional = nullable)] Option<i32>,
);

#[test]
fn tuple_nullable() {
    let cfg = Config::from_env();
    assert_eq!(
        NullableTuple::inline(&cfg),
        "[number, string, string, (number | null)?, (number)?, (number | null)?]"
    );
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/", optional_fields)]
enum OptionalFieldsEnum {
    A { a: Option<i32> },
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/", optional_fields = nullable)]
enum OptionalFieldsEnumVariantOverride {
    // Disable `nullable`
    #[ts(optional_fields)]
    A { a: Option<i32> },

    // Disable `optional_fields`
    #[ts(optional_fields = false)]
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/", optional_fields)]
enum OptionalFieldsEnumNotNullableVariantOverride {
    // Disable `nullable`
    #[ts(optional_fields = nullable)]
    A { a: Option<i32> },

    // Disable `optional_fields`
    #[ts(optional_fields = false)]
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "optional_field/", optional_fields, tag = "type")]
enum OptionalFieldsTaggedEnum {
    A { a: Option<i32> },
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[ts(
    export,
    export_to = "optional_field/",
    optional_fields,
    tag = "type",
    content = "data"
)]
enum OptionalFieldsExternallyTaggedEnum {
    A { a: Option<i32> },
    B { b: String, c: Option<bool> },
}

#[test]
fn optional_fields_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalFieldsEnum::inline(&cfg),
        r#"{ "A": { a?: number, } } | { "B": { b: string, c?: boolean, } }"#
    );

    assert_eq!(
        OptionalFieldsEnumVariantOverride::inline(&cfg),
        r#"{ "A": { a?: number, } } | { "B": { b: string, c: boolean | null, } }"#
    );

    assert_eq!(
        OptionalFieldsEnumNotNullableVariantOverride::inline(&cfg),
        r#"{ "A": { a?: number | null, } } | { "B": { b: string, c: boolean | null, } }"#
    );

    assert_eq!(
        OptionalFieldsTaggedEnum::inline(&cfg),
        r#"{ "type": "A", a?: number, } | { "type": "B", b: string, c?: boolean, }"#
    );

    assert_eq!(
        OptionalFieldsExternallyTaggedEnum::inline(&cfg),
        r#"{ "type": "A", "data": { a?: number, } } | { "type": "B", "data": { b: string, c?: boolean, } }"#
    );
}
