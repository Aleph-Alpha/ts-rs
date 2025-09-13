use ts_rs::TS;

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
    A,
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
    assert_eq!(Foo::decl(), "enum Foo { \"A\" = 1, \"B\" = 2 }");
    assert_eq!(Bar::decl(), "enum Bar { \"A\" = 1, \"B\" }");
    assert_eq!(Baz::decl(), "enum Baz { \"A\", \"B\" }");
    assert_eq!(Biz::decl(), "enum Biz { \"A\" = \"A\", \"B\" = \"B\" }");
    assert_eq!(SnakeCase::decl(), "enum SnakeCase { \"enum_variant_foo\" = \"enum_variant_foo\", \"enum_variant_bar\" = \"enum_variant_bar\" }");
    assert_eq!(CamelCase::decl(), "enum CamelCase { \"enumVariantFoo\" = \"enumVariantFoo\", \"enumVariantBar\" = \"enumVariantBar\" }");
    assert_eq!(KebabCase::decl(), "enum KebabCase { \"enum-variant-foo\" = \"enum-variant-foo\", \"enum-variant-bar\" = \"enum-variant-bar\" }");
}

#[test]
fn native_ts_enum_repr_inline() {
    assert_eq!(Foo::inline(), "1 | 2");
    assert_eq!(Bar::inline(), "1 | 2");
    assert_eq!(Baz::inline(), "0 | 1");

    assert_eq!(Biz::inline(), r#""A" | "B""#);
    assert_eq!(
        SnakeCase::inline(),
        r#""enum_variant_foo" | "enum_variant_bar""#
    );
    assert_eq!(
        CamelCase::inline(),
        r#""enumVariantFoo" | "enumVariantBar""#
    );
    assert_eq!(
        KebabCase::inline(),
        r#""enum-variant-foo" | "enum-variant-bar""#
    );
}
