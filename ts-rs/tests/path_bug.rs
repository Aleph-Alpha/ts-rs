#![allow(dead_code)]
use ts_rs::{TS, output_path};

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

    assert!(output_path::<Foo>().unwrap().is_file());
    assert!(output_path::<Bar>().unwrap().is_file());
}
