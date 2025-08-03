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
#[ts(export, export_to = "repr_enum/",repr(enum = name))]
enum Biz {
    A,
    B,
}

#[test]
fn native_ts_enum_repr() {
    assert_eq!(Foo::decl(), "enum Foo { \"A\" = 1, \"B\" = 2 }");
    assert_eq!(Bar::decl(), "enum Bar { \"A\" = 1, \"B\" }");
    assert_eq!(Baz::decl(), "enum Baz { \"A\", \"B\" }");
    assert_eq!(Biz::decl(), "enum Biz { \"A\" = \"A\", \"B\" = \"B\" }");
}
