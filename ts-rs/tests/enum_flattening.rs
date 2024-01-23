use serde::Serialize;
use ts_rs::TS;

#[test]
#[cfg(feature = "serde-compat")]
fn externally_tagged() {
    #[derive(Serialize, TS)]
    struct Foo {
        #[serde(flatten)]
        baz: Bar,
    }

    #[derive(Serialize, TS)]
    #[allow(dead_code)]
    enum Bar {
        Baz { a: i32, a2: String, },
        Biz { b: bool },
        Buz { c: String },
    }

    assert_eq!(Foo::inline(), r#"{ "Baz": { a: number, a2: string, }, } | { "Biz": { b: boolean, }, } | { "Buz": { c: string, }, }"#)
}

#[test]
#[cfg(feature = "serde-compat")]
fn adjacently_tagged() {
    #[derive(Serialize, TS)]
    struct Foo {
        #[serde(flatten)]
        baz: Bar,
    }

    #[derive(Serialize, TS)]
    #[allow(dead_code)]
    #[serde(tag = "type", content = "stuff")]
    enum Bar {
        Baz { a: i32, a2: String, },
        Biz { b: bool },
        Buz { c: String },
    }

    assert_eq!(Foo::inline(), r#"{ "type": "Baz", "stuff": { a: number, a2: string, }, } | { "type": "Biz", "stuff": { b: boolean, }, } | { "type": "Buz", "stuff": { c: string, }, }"#)

}

#[test]
#[cfg(feature = "serde-compat")]
fn internally_tagged() {
    #[derive(Serialize, TS)]
    struct Foo {
        #[serde(flatten)]
        baz: Bar,
    }

    #[derive(Serialize, TS)]
    #[allow(dead_code)]
    #[serde(tag = "type")]
    enum Bar {
        Baz { a: i32, a2: String, },
        Biz { b: bool },
        Buz { c: String },
    }

    assert_eq!(Foo::inline(), r#"{ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, }"#)
}

#[test]
#[cfg(feature = "serde-compat")]
fn untagged() {
    #[derive(Serialize, TS)]
    struct Foo {
        #[serde(flatten)]
        baz: Bar,
    }

    #[derive(Serialize, TS)]
    #[allow(dead_code)]
    #[serde(untagged)]
    enum Bar {
        Baz { a: i32, a2: String, },
        Biz { b: bool },
        Buz { c: String },
    }

    assert_eq!(Foo::inline(), r#"{ a: number, a2: string, } | { b: boolean, } | { c: string, }"#)
}
