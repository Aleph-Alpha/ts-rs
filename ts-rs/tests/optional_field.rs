#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

#[test]
fn in_struct() {
    #[derive(Serialize, TS)]
    struct Optional {
        #[ts(optional)]
        a: Option<i32>,
        #[ts(optional = nullable)]
        b: Option<i32>,
        c: Option<i32>,
    }

    let a = "a?: number";
    let b = "b?: number | null";
    let c = "c: number | null";
    assert_eq!(Optional::inline(), format!("{{ {a}, {b}, {c}, }}"));
}

#[test]
fn in_enum() {
    #[derive(Serialize, TS)]
    enum Optional {
        A {
            #[ts(optional)]
            a: Option<i32>,
        },
        B {
            b: Option<String>,
        },
    }

    assert_eq!(
        Optional::inline(),
        r#"{ "A": { a?: number, } } | { "B": { b: string | null, } }"#
    );
}

#[test]
fn flatten() {
    #[derive(Serialize, TS)]
    struct Optional {
        #[ts(optional)]
        a: Option<i32>,
        #[ts(optional = nullable)]
        b: Option<i32>,
        c: Option<i32>,
    }

    #[derive(Serialize, TS)]
    struct Flatten {
        #[ts(flatten)]
        x: Optional,
    }

    assert_eq!(Flatten::inline(), Optional::inline());
}

#[test]
fn inline() {
    #[derive(Serialize, TS)]
    struct Optional {
        #[ts(optional)]
        a: Option<i32>,
        #[ts(optional = nullable)]
        b: Option<i32>,
        c: Option<i32>,
    }

    #[derive(Serialize, TS)]
    struct Inline {
        #[ts(inline)]
        x: Optional,
    }

    let a = "a?: number";
    let b = "b?: number | null";
    let c = "c: number | null";
    assert_eq!(Inline::inline(), format!("{{ x: {{ {a}, {b}, {c}, }}, }}"));
}
