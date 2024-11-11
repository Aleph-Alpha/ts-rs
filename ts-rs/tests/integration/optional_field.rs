#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

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
    assert_eq!(OptionalInStruct::inline(), format!("{{ {a}, {b}, {c}, }}"));
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
    assert_eq!(
        OptionalInEnum::inline(),
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
    assert_eq!(Flatten::inline(), OptionalFlatten::inline());
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
    assert_eq!(Inline::inline(), format!("{{ x: {{ {a}, {b}, {c}, }}, }}"));
}

type Foo = Option<i32>;
type Bar<T> = Option<T>;

#[derive(TS)]
#[ts(export, export_to = "optional_field/", optional)]
struct OptionalStruct {
    a: Option<i32>,
    b: Option<i32>,

    #[ts(optional = nullable)]
    c: Option<i32>,

    d: i32,

    e: Foo,
    f: Bar<i32>,
}

#[test]
fn struct_optional() {
    assert_eq!(
        OptionalStruct::inline(),
        format!(
            "{{ a?: number, b?: number, c?: number | null, d: number, e?: number, f?: number, }}"
        )
    )
}
