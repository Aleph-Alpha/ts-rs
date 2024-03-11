#![allow(unused)]

use ts_rs::TS;

trait Driver {
    type Info: TS;
}

#[derive(TS)]
#[ts(export, export_to = "concrete_generic/")]
struct Foo {
    x: i32
}

struct TsDriver;
impl Driver for TsDriver {
    type Info = String;
}

#[derive(TS)]
#[ts(export, export_to = "concrete_generic/")]
struct OtherDriver;
impl Driver for OtherDriver {
    type Info = Foo;
}

#[derive(TS)]
#[ts(export, export_to = "concrete_generic/", concrete(T = TsDriver))]
struct MyStruct<T: Driver> {
    u: T::Info,
}

#[derive(TS)]
#[ts(export, export_to = "concrete_generic/", concrete(T = OtherDriver))]
struct OtherStruct<T: Driver + TS> {
    u: T::Info,
    x: T,
}


#[test]
fn concrete_generic_param() {
    assert_eq!(
        MyStruct::<TsDriver>::decl(),
        "type MyStruct = { u: string, };"
    );
    assert_eq!(
        OtherStruct::<OtherDriver>::decl_concrete(),
        "type OtherStruct = { u: Foo, x: OtherDriver, };"
    );
}
