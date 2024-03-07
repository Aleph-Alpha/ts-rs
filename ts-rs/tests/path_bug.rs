#![allow(dead_code)]
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "../ts-rs/tests-out/path_bug/")]
struct Foo {
    bar: Bar,
}

#[derive(TS)]
#[ts(export_to = "tests-out/path_bug/aaa/")]
struct Bar {
    i: i32,
}

#[test]
fn path_bug() {
    export_bindings_foo();

    assert!(Foo::output_path().unwrap().is_file());
    assert!(Bar::output_path().unwrap().is_file());
}
