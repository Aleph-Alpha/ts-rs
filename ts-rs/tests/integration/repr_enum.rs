use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", repr(enum))]
enum Foo {
    A = 1,
    B = 2,
}

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", repr(enum))]
enum Bar {
    A = 1,
    B,
}

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", repr(enum))]
enum Baz {
    A,
    B,
}

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", repr(enum = name))]
enum Biz {
    // The discriminant is ignored when using `repr(enum = name)`
    A = 0,
    B,
}

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", rename_all = "snake_case", repr(enum = name))]
enum SnakeCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", rename_all = "camelCase", repr(enum = name))]
enum CamelCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[derive(TS)]
#[ts(export, export_to = "repr_enum/", rename_all = "kebab-case", repr(enum = name))]
enum KebabCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[test]
fn native_ts_enum_repr() {
    let cfg = Config::from_env();
    assert_eq!(
        Foo::decl(&cfg),
        "enum Foo { \"A\" = 1, \"B\" = 2 }"
    );
    assert_eq!(
        Bar::decl(&cfg),
        "enum Bar { \"A\" = 1, \"B\" }"
    );
    assert_eq!(
        Baz::decl(&cfg),
        "enum Baz { \"A\", \"B\" }"
    );
    assert_eq!(
        Biz::decl(&cfg),
        "enum Biz { \"A\" = \"A\", \"B\" = \"B\" }"
    );
    assert_eq!(
        SnakeCase::decl(&cfg),
        "enum SnakeCase { \"enum_variant_foo\" = \"enum_variant_foo\", \"enum_variant_bar\" = \"enum_variant_bar\" }"
    );
    assert_eq!(
        CamelCase::decl(&cfg),
        "enum CamelCase { \"enumVariantFoo\" = \"enumVariantFoo\", \"enumVariantBar\" = \"enumVariantBar\" }"
    );
    assert_eq!(
        KebabCase::decl(&cfg),
        "enum KebabCase { \"enum-variant-foo\" = \"enum-variant-foo\", \"enum-variant-bar\" = \"enum-variant-bar\" }"
    );
}

#[test]
fn native_ts_enum_repr_inline() {
    let cfg = Config::from_env();
    assert_eq!(Foo::inline(&cfg), "1 | 2");
    assert_eq!(Bar::inline(&cfg), "1 | 2");
    assert_eq!(Baz::inline(&cfg), "0 | 1");

    assert_eq!(Biz::inline(&cfg), r#""A" | "B""#);
    assert_eq!(
        SnakeCase::inline(&cfg),
        r#""enum_variant_foo" | "enum_variant_bar""#
    );
    assert_eq!(
        CamelCase::inline(&cfg),
        r#""enumVariantFoo" | "enumVariantBar""#
    );
    assert_eq!(
        KebabCase::inline(&cfg),
        r#""enum-variant-foo" | "enum-variant-bar""#
    );
}
