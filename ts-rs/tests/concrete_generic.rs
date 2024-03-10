#![allow(unused)]

use ts_rs::TS;

trait Driver {
    type Info: TS;
}

struct TsDriver;
impl Driver for TsDriver {
    type Info = String;
}

struct OtherDriver;
impl Driver for OtherDriver {
    type Info = i32;
}

#[derive(TS)]
#[ts(export)]
struct MyStruct<#[ts(concrete = "TsDriver")] T: Driver> {
    u: T::Info,
}

#[test]
fn concrete_generic_param() {
    assert_eq!(
        MyStruct::<TsDriver>::decl(),
        "type MyStruct = { u: string, };"
    );
}
