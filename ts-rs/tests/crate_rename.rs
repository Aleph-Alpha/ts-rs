#![allow(dead_code)]

use ts_rs as reexport;

#[derive(ts_rs::TS)]
#[ts(crate_rename = "reexport")]
struct TestStruct {
    hello: String,
    world: i32,
}

#[test]
fn reexported_impl_works() {
    let ts = <TestStruct as reexport::TS>::decl();
    assert_eq!(ts, "type TestStruct = { hello: string, world: number, };");
}
