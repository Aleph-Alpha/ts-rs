use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "same_file_export/")]
struct DepA {
    foo: i32,
}

#[derive(TS)]
#[ts(export, export_to = "same_file_export/")]
struct DepB {
    foo: i32,
}

#[derive(TS)]
#[ts(export, export_to = "same_file_export/", repr(enum = const_object))]
enum DepC {
    A,
    B,
    C,
}

#[derive(TS)]
#[ts(export, export_to = "same_file_export/types.ts")]
struct A {
    foo: DepA,
}

#[derive(TS)]
#[ts(export, export_to = "same_file_export/types.ts")]
struct B {
    foo: DepB,
}

#[derive(TS)]
#[ts(export, export_to = "same_file_export/types.ts")]
struct C {
    foo: DepA,
    bar: DepB,
    biz: B,
    baz: DepC,
}

#[derive(TS)]
#[ts(export, export_to = "same_file_export/types.ts", repr(enum = const_object))]
enum ConstEnumB {
    B,
}
