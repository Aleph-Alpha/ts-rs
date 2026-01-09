use serde::Serialize;
use ts_rs::TS;

#[test]
fn optional_flatten_enum_adds_empty_object() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        flattened: Option<Enum>,
    }

    assert_eq!(
        T::inline_flattened(),
        r#"{ a: string, } & ({ "firstOption": string; "secondOption"?: never } | { "secondOption": boolean; "firstOption"?: never } | { "firstOption"?: never; "secondOption"?: never })"#
    );
}

#[test]
fn optional_flatten_unit_variants_adds_empty_object() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        First,
        Second,
        Third,
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        status: Option<Enum>,
    }

    assert_eq!(
        T::inline_flattened(),
        r#"{ a: string, } & ("first" | "second" | "third" | { "first"?: never; "second"?: never; "third"?: never })"#
    );
}

#[test]
fn optional_flatten_mixed_variants_adds_empty_object() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        Unit,
        Tuple(i32, String),
        Struct { x: i32, y: String },
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        data: Option<Enum>,
    }

    assert_eq!(
        T::inline_flattened(),
        r#"{ a: string, } & ("unit" | { "tuple": [number, string]; "struct"?: never; "unit"?: never } | { "struct": { x: number, y: string, }; "tuple"?: never; "unit"?: never } | { "unit"?: never; "tuple"?: never; "struct"?: never })"#
    );
}

#[test]
fn optional_flatten_with_nested_objects_adds_empty_object() {
    #[derive(TS, Serialize)]
    struct Inner {
        value: i32,
    }

    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        First(Inner),
        Second(Inner),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        nested: Option<Enum>,
    }

    assert_eq!(
        T::inline_flattened(),
        r#"{ a: string, } & ({ "first": Inner; "second"?: never } | { "second": Inner; "first"?: never } | { "first"?: never; "second"?: never })"#
    );
}

#[test]
fn multiple_optional_flattened_enums_each_add_empty_object() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum EnumA {
        OptionA(String),
    }

    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum EnumB {
        OptionB(i32),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        a_enum: Option<EnumA>,
        #[serde(flatten)]
        b_enum: Option<EnumB>,
    }

    let result = T::inline_flattened();

    assert!(result.contains(r#"{ "optionA": string; "optionA"?: never } | { "optionA"?: never }"#));
    assert!(result.contains(r#"{ "optionB": number; "optionB"?: never } | { "optionB"?: never }"#));
}

#[test]
fn optional_flatten_with_rename_all_kebab_case() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "kebab-case")]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        flattened: Option<Enum>,
    }

    let result = T::inline_flattened();

    assert!(result.contains(r#""first-option": string"#));
    assert!(result.contains(r#""second-option": boolean"#));
    assert!(result.contains(r#""first-option"?: never"#));
    assert!(result.contains(r#""second-option"?: never"#));
}

#[test]
fn optional_flatten_single_variant_adds_empty_object() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        Only(String),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        single: Option<Enum>,
    }

    assert_eq!(
        T::inline_flattened(),
        r#"{ a: string, } & ({ "only": string } | { "only"?: never })"#
    );
}

#[test]
fn non_optional_flatten_enum_is_unchanged() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        flattened: Enum,
    }

    assert_eq!(
        T::inline_flattened(),
        r#"{ a: string, } & ({ "firstOption": string } | { "secondOption": boolean })"#
    );
}
