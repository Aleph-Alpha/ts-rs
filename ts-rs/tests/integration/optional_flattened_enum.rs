use serde::Serialize;
use ts_rs::TS;

#[test]
fn two_variant_enum() {
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
        T::optional_inline_flattened(),
        r#"{ a: string, } & ({ "firstOption": string; "secondOption"?: never } | { "secondOption": boolean; "firstOption"?: never } | { "firstOption"?: never; "secondOption"?: never })"#
    );
}

#[test]
fn three_variant_enum() {
    #[derive(TS, Serialize)]
    #[serde(rename_all = "camelCase")]
    enum Enum {
        FirstOption(String),
        SecondOption(bool),
        ThirdOption(usize),
    }

    #[derive(TS, Serialize)]
    struct T {
        a: String,
        #[serde(flatten)]
        flattened: Option<Enum>,
    }

    assert_eq!(
        T::optional_inline_flattened(),
        r#"{ a: string, } & ({ "firstOption": string; "secondOption"?: never; "thirdOption"?: never } | { "secondOption": boolean; "firstOption"?: never; "thirdOption"?: never } | { "thirdOption": number; "firstOption"?: never; "secondOption"?: never } | { "firstOption"?: never; "secondOption"?: never; "thirdOption"?: never })"#
    );
}

#[test]
#[should_panic(expected = "Enum cannot be flattened")]
fn unit_variants() {
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

    // "first" | "second" | "third" isn't valid
    assert_eq!(
        T::optional_inline_flattened(),
        r#"{ a: string, } & ("first" | "second" | "third" | { "first"?: never; "second"?: never; "third"?: never })"#
    );
}

#[test]
#[should_panic(expected = "Enum cannot be flattened")]
fn mixed_variant_types() {
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

    // "unit" isn't valid
    assert_eq!(
        T::optional_inline_flattened(),
        r#"{ a: string, } & ("unit" | { "tuple": [number, string]; "unit"?: never; "struct"?: never } | { "struct": { x: number, y: string, }; "unit"?: never; "tuple"?: never } | { "unit"?: never; "tuple"?: never; "struct"?: never })"#
    );
}

#[test]
fn nested_structs() {
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
        T::optional_inline_flattened(),
        r#"{ a: string, } & ({ "first": Inner; "second"?: never } | { "second": Inner; "first"?: never } | { "first"?: never; "second"?: never })"#
    );
}

#[test]
fn kebab_case_renaming() {
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

    let result = T::optional_inline_flattened();

    assert!(result.contains(r#""first-option": string"#));
    assert!(result.contains(r#""second-option": boolean"#));
    assert!(result.contains(r#""first-option"?: never"#));
    assert!(result.contains(r#""second-option"?: never"#));
}

#[test]
fn single_variant_enum() {
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
        T::optional_inline_flattened(),
        r#"{ a: string, } & ({ "only": string; } | { "only"?: never })"#
    );
}

#[test]
fn original_non_optional_enum() {
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
        T::optional_inline_flattened(),
        r#"{ a: string, } & ({ "firstOption": string } | { "secondOption": boolean })"#
    );
}
