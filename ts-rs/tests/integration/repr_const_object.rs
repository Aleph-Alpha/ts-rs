use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "repr_const_object/", repr(enum = const_object))]
enum Foo {
    A = 1,
    B = 2,
}

#[derive(TS)]
#[ts(export, export_to = "repr_const_object/", repr(enum = const_object))]
enum Bar {
    A = 1,
    B,
}

#[derive(TS)]
#[ts(export, export_to = "repr_const_object/", repr(enum = const_object))]
enum Baz {
    A,
    B,
}

#[derive(TS)]
#[ts(export, export_to = "repr_const_object/", rename_all = "snake_case", repr(enum = const_object))]
enum SnakeCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[derive(TS)]
#[ts(export, export_to = "repr_const_object/", rename_all = "camelCase", repr(enum = const_object))]
enum CamelCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[derive(TS)]
#[ts(export, export_to = "repr_const_object/", rename_all = "kebab-case", repr(enum = const_object))]
enum KebabCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[test]
fn native_ts_enum_repr_const_object() {
    let cfg = Config::from_env();
    assert_eq!(
        Foo::decl(&cfg),
        "const Foo = { \"A\": 1, \"B\": 2 } as const;\nexport type Foo = (typeof Foo)[keyof typeof Foo];"
    );
    assert_eq!(
        Bar::decl(&cfg),
        "const Bar = { \"A\": 1, \"B\": 2 } as const;\nexport type Bar = (typeof Bar)[keyof typeof Bar];"
    );
    assert_eq!(
        Baz::decl(&cfg),
        "const Baz = { \"A\": \"A\", \"B\": \"B\" } as const;\nexport type Baz = (typeof Baz)[keyof typeof Baz];"
    );
    assert_eq!(
        SnakeCase::decl(&cfg),
        "const SnakeCase = { \"enum_variant_foo\": \"enum_variant_foo\", \"enum_variant_bar\": \"enum_variant_bar\" } as const;\nexport type SnakeCase = (typeof SnakeCase)[keyof typeof SnakeCase];"
    );
    assert_eq!(
        CamelCase::decl(&cfg),
        "const CamelCase = { \"enumVariantFoo\": \"enumVariantFoo\", \"enumVariantBar\": \"enumVariantBar\" } as const;\nexport type CamelCase = (typeof CamelCase)[keyof typeof CamelCase];"
    );
    assert_eq!(
        KebabCase::decl(&cfg),
        "const KebabCase = { \"enum-variant-foo\": \"enum-variant-foo\", \"enum-variant-bar\": \"enum-variant-bar\" } as const;\nexport type KebabCase = (typeof KebabCase)[keyof typeof KebabCase];"
    );
}

#[test]
fn native_ts_enum_repr_const_object_inline() {
    let cfg = Config::from_env();
    assert_eq!(Foo::inline(&cfg), "1 | 2");
    assert_eq!(Bar::inline(&cfg), "1 | 2");
    assert_eq!(Baz::inline(&cfg), "\"A\" | \"B\"");
    assert_eq!(
        SnakeCase::inline(&cfg),
        "\"enum_variant_foo\" | \"enum_variant_bar\""
    );
    assert_eq!(
        CamelCase::inline(&cfg),
        "\"enumVariantFoo\" | \"enumVariantBar\""
    );
    assert_eq!(
        KebabCase::inline(&cfg),
        "\"enum-variant-foo\" | \"enum-variant-bar\""
    );
}
