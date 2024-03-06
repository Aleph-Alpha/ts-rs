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
    Foo::export().unwrap();
    Bar::export().unwrap();

    let base = std::env::current_dir().unwrap();
    assert!(base.join("./tests-out/path_bug/Foo.ts").is_file());
    assert!(base.join("./tests-out/path_bug/aaa/Bar.ts").is_file());
}