#![allow(dead_code)]

use std::time::Instant;

use ts_rs::TS;

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
    }

    assert_eq!(
        Override::inline(),
        "{ a: number, x: { a: number, b: number, c: number, }, }"
    )
}

#[test]
fn enum_variants() {
    #[derive(TS)]
    enum OverrideEnum {
        A(#[ts(as = "ExternalTypeDef")] Instant),
        B {
            #[ts(as = "ExternalTypeDef")]
            x: i32,
            y: i32,
            z: i32,
        },
    }

    assert_eq!(
        OverrideEnum::inline(),
        r#"{ "A": ExternalTypeDef } | { "B": { x: ExternalTypeDef, y: number, z: number, } }"#
    )
}
