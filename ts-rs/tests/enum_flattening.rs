#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use ts_rs::TS;

#[test]
fn externally_tagged() {
    #[allow(dead_code)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize, TS))]
    #[cfg_attr(not(feature = "serde-compat"), derive(TS))]
    struct Foo {
        qux: i32,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        baz: Bar,
        biz: Option<String>,
    }

    #[cfg_attr(feature = "serde-compat", derive(Serialize, TS))]
    #[cfg_attr(not(feature = "serde-compat"), derive(TS))]
    #[allow(dead_code)]
    enum Bar {
        Baz { a: i32, a2: String },
        Biz { b: bool },
        Buz { c: String, d: Option<i32> },
    }

    assert_eq!(
        Foo::inline(),
        r#"{ qux: number, biz: string | null, } & ({ "Baz": { a: number, a2: string, } } | { "Biz": { b: boolean, } } | { "Buz": { c: string, d: number | null, } })"#
    )
}

#[test]
fn adjacently_tagged() {
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[derive(TS)]
    struct Foo {
        one: i32,
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        baz: Bar,
        qux: Option<String>,
    }

    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[derive(TS)]
    #[allow(dead_code)]
    #[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "stuff"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(tag = "type", content = "stuff"))]
    enum Bar {
        Baz {
            a: i32,
            a2: String,
        },
        Biz {
            b: bool,
        },

        #[cfg_attr(feature = "serde-compat", serde(untagged))]
        #[cfg_attr(not(feature = "serde-compat"), ts(untagged))]
        Buz {
            c: String,
            d: Option<i32>,
        },
    }

    assert_eq!(
        Foo::inline(),
        r#"{ one: number, qux: string | null, } & ({ "type": "Baz", "stuff": { a: number, a2: string, } } | { "type": "Biz", "stuff": { b: boolean, } } | { c: string, d: number | null, })"#
    )
}

#[test]
fn internally_tagged() {
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[derive(TS)]
    struct Foo {
        qux: Option<String>,

        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        baz: Bar,
    }

    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[derive(TS)]
    #[allow(dead_code)]
    #[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(tag = "type"))]
    enum Bar {
        Baz { a: i32, a2: String },
        Biz { b: bool },
        Buz { c: String, d: Option<i32> },
    }

    assert_eq!(
        Foo::inline(),
        r#"{ qux: string | null, } & ({ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, d: number | null, })"#
    )
}

#[test]
fn untagged() {
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[derive(TS)]
    struct Foo {
        #[cfg_attr(feature = "serde-compat", serde(flatten))]
        #[cfg_attr(not(feature = "serde-compat"), ts(flatten))]
        baz: Bar,
    }

    #[derive(TS)]
    #[cfg_attr(feature = "serde-compat", derive(Serialize))]
    #[cfg_attr(feature = "serde-compat", serde(untagged))]
    #[cfg_attr(not(feature = "serde-compat"), ts(untagged))]
    enum Bar {
        Baz { a: i32, a2: String },
        Biz { b: bool },
        Buz { c: String },
    }

    assert_eq!(
        Foo::inline(),
        r#"{ a: number, a2: string, } | { b: boolean, } | { c: string, }"#
    )
}
