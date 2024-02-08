#![allow(dead_code)]

use std::{
    cell::UnsafeCell, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicPtr, time::Instant,
};

use ts_rs::TS;

type Unsupported = UnsafeCell<MaybeUninit<NonNull<AtomicPtr<i32>>>>;

#[derive(TS)]
struct ExternalTypeDef {
    a: i32,
    b: i32,
    c: i32,
}

#[test]
fn struct_properties() {
    #[derive(TS)]
    struct Override {
        a: i32,
        #[ts(as = "ExternalTypeDef")]
        #[ts(inline)]
        x: Instant,
        // here, 'as' just behaves like 'type' (though it adds a dependency!)
        #[ts(as = "ExternalTypeDef")]
        y: Unsupported,
    }

    assert_eq!(
        Override::inline(),
        "{ a: number, x: { a: number, b: number, c: number, }, y: ExternalTypeDef, }"
    );
    assert!(Override::dependencies()
        .iter()
        .any(|d| d.ts_name == "ExternalTypeDef"));
}

#[test]
fn enum_variants() {
    #[derive(TS)]
    enum OverrideEnum {
        A(#[ts(as = "ExternalTypeDef")] Instant),
        B {
            #[ts(as = "ExternalTypeDef")]
            x: Unsupported,
            y: i32,
            z: i32,
        },
    }

    assert_eq!(
        OverrideEnum::inline(),
        r#"{ "A": ExternalTypeDef } | { "B": { x: ExternalTypeDef, y: number, z: number, } }"#
    )
}

#[test]
fn complex() {
    #[derive(TS)]
    struct Outer {
        #[ts(as = "Option<ExternalTypeDef>")]
        #[ts(optional = nullable, inline)]
        x: Unsupported,
        #[ts(as = "Option<ExternalTypeDef>")]
        #[ts(optional = nullable)]
        y: Unsupported,
    }

    let external = ExternalTypeDef::inline();
    assert_eq!(
        Outer::inline(),
        format!(r#"{{ x?: {external} | null, y?: ExternalTypeDef | null, }}"#)
    )
}
