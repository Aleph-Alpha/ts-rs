#![allow(dead_code)]
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "path_bug/aaa/")]
struct Foo {
    bar: Bar,
}

#[derive(TS)]
#[ts(export_to = "../bindings/path_bug/")]
struct Bar {
    i: i32,
}

#[test]
fn path_bug() {
    let cfg = Config::from_env();
    export_bindings_foo();

    assert!(cfg.out_dir().join(Foo::output_path().unwrap()).is_file());
    assert!(cfg.out_dir().join(Bar::output_path().unwrap()).is_file());
}
