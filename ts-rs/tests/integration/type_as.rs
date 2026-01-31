#![allow(dead_code)]

use std::{
    cell::UnsafeCell, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicPtr, time::Instant,
};

use serde::Serialize;
use ts_rs::{Config, TS};

type Unsupported = UnsafeCell<MaybeUninit<NonNull<AtomicPtr<i32>>>>;

#[derive(TS)]
#[ts(export, export_to = "type_as/")]
struct ExternalTypeDef {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(TS)]
#[ts(export, export_to = "type_as/")]
struct Override {
    a: i32,
    #[ts(as = "ExternalTypeDef")]
    #[ts(inline)]
    x: Instant,
    // here, 'as' just behaves like 'type' (though it adds a dependency!)
    #[ts(as = "ExternalTypeDef")]
    y: Unsupported,
    #[ts(as = "(i32, ExternalTypeDef, i32)")]
    z: Unsupported,
}

#[test]
fn struct_properties() {
    let cfg = Config::from_env();
    assert_eq!(
        Override::inline(&cfg),
        "{ \
           a: number, \
           x: { a: number, b: number, c: number, }, \
           y: ExternalTypeDef, \
           z: [number, ExternalTypeDef, number], \
        }"
    );
    assert!(Override::dependencies(&cfg)
        .iter()
        .any(|d| d.ts_name == "ExternalTypeDef"));
}

#[derive(TS)]
#[ts(export, export_to = "type_as/")]
enum OverrideEnum {
    A(#[ts(as = "ExternalTypeDef")] Instant),
    B {
        #[ts(as = "ExternalTypeDef")]
        x: Unsupported,
        y: i32,
        z: i32,
    },
}

mod deser {
    use serde::{Serialize, Serializer};

    use super::Instant;
    pub fn serialize<S: Serializer>(field: &Instant, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Foo {
            x: i32,
        }
        Foo { x: 0 }.serialize(serializer)
    }
}

#[derive(TS)]
struct OverrideVariantDef {
    x: i32,
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "type_as/")]
enum OverrideVariant {
    #[ts(as = "OverrideVariantDef")]
    #[serde(with = "deser")]
    A {
        x: Instant,
    },
    B {
        y: i32,
        z: i32,
    },
}

#[test]
fn enum_variants() {
    let a = OverrideVariant::A { x: Instant::now() };
    assert_eq!(serde_json::to_string(&a).unwrap(), r#"{"A":{"x":0}}"#);
    let cfg = Config::from_env();
    assert_eq!(
        OverrideEnum::inline(&cfg),
        r#"{ "A": ExternalTypeDef } | { "B": { x: ExternalTypeDef, y: number, z: number, } }"#
    );

    assert_eq!(
        OverrideVariant::inline(&cfg),
        r#"{ "A": OverrideVariantDef } | { "B": { y: number, z: number, } }"#
    );
}

#[derive(TS)]
#[ts(export, export_to = "type_as/")]
struct Outer {
    #[ts(as = "Option<ExternalTypeDef>")]
    #[ts(optional = nullable, inline)]
    x: Unsupported,
    #[ts(as = "Option<ExternalTypeDef>")]
    #[ts(optional = nullable)]
    y: Unsupported,
}

#[test]
fn complex() {
    let cfg = Config::from_env();
    let external = ExternalTypeDef::inline(&cfg);
    assert_eq!(
        Outer::inline(&cfg),
        format!(r#"{{ x?: {external} | null, y?: ExternalTypeDef | null, }}"#)
    )
}
