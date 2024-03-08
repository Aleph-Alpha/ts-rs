#![allow(dead_code)]

use std::{
    cell::UnsafeCell, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicPtr, time::Instant,
};

use ts_rs::TS;

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
}

#[test]
fn struct_properties() {
    assert_eq!(
        Override::inline(),
        "{ a: number, x: { a: number, b: number, c: number, }, y: ExternalTypeDef, }"
    );
    assert!(Override::dependencies()
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

#[test]
fn enum_variants() {
    assert_eq!(
        OverrideEnum::inline(),
        r#"{ "A": ExternalTypeDef } | { "B": { x: ExternalTypeDef, y: number, z: number, } }"#
    )
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
    let external = ExternalTypeDef::inline();
    assert_eq!(
        Outer::inline(),
        format!(r#"{{ x?: {external} | null, y?: ExternalTypeDef | null, }}"#)
    )
}
