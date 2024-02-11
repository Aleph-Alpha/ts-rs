#![allow(dead_code, unused_imports)]

use std::error::Error;
use serde::Serialize;
use ts_rs::TS;

struct Unsupported;

#[test]
fn simple() {
    #[derive(TS)]
    struct Skip {
        a: i32,
        b: i32,
        #[ts(skip)]
        c: String,
        #[ts(skip)]
        d: Box<dyn Error>
    }

    assert_eq!(Skip::inline(), "{ a: number, b: number, }");
}

#[test]
fn externally_tagged() {
    #[cfg_attr(feature = "serde-compat", derive(Serialize, TS))]
    #[cfg_attr(not(feature = "serde-compat"), derive(TS))]
    enum Externally {
        A (
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            Unsupported,
        ),
        B(
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            Unsupported,
            i32
        ),
        C {
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            x: Unsupported,
        },
        D {
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            x: Unsupported,
            y: i32
        },
    }

    // TODO: variant C should probably not generate `{}`
    assert_eq!(
        Externally::decl(),
        r#"type Externally = "A" | { "B": [number] } | { "C": {  } } | { "D": { y: number, } };"#
    );
}

#[test]
fn internally_tagged() {
    #[cfg_attr(feature = "serde-compat", derive(Serialize, TS))]
    #[cfg_attr(not(feature = "serde-compat"), derive(TS))]
    #[cfg_attr(feature = "serde-compat", serde(tag = "t"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(tag = "t"))]
    enum Internally {
        A (
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            Unsupported,
        ),
        B {
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            x: Unsupported,
        },
        C {
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            x: Unsupported,
            y: i32,
        },
    }

    assert_eq!(
        Internally::decl(),
        r#"type Internally = { "t": "A" } | { "t": "B",  } | { "t": "C", y: number, };"#
    );
}

#[test]
fn adjacently_tagged() {
    #[cfg_attr(feature = "serde-compat", derive(Serialize, TS))]
    #[cfg_attr(not(feature = "serde-compat"), derive(TS))]
    #[cfg_attr(feature = "serde-compat", serde(tag = "t", content = "c"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(tag = "t", content = "c"))]
    enum Adjacently {
        A (
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            Unsupported,
        ),
        B(
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            Unsupported,
            i32
        ),
        C {
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            x: Unsupported,
        },
        D {
            #[cfg_attr(feature = "serde-compat", serde(skip))]
            #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
            x: Unsupported,
            y: i32
        },

    }

    // TODO: variant C should probably not generate `{ .., "c": { } }`
    assert_eq!(
        Adjacently::decl(),
        r#"type Adjacently = { "t": "A" } | { "t": "B", "c": [number] } | { "t": "C", "c": {  } } | { "t": "D", "c": { y: number, } };"#
    );
}
