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
#[ts(export_to = "same_file_export/types.ts")]
struct A {
    foo: DepA,
}

#[derive(TS)]
#[ts(export_to = "same_file_export/types.ts")]
struct B {
    foo: DepB,
}

#[derive(TS)]
#[ts(export_to = "same_file_export/types.ts")]
struct C {
    foo: DepA,
    bar: DepB,
    biz: B,
}

#[test]
fn all_types_exported() {
    A::export_all().unwrap();
    B::export_all().unwrap();
    C::export_all().unwrap();

    let contents = std::fs::read_to_string(&A::default_output_path().unwrap()).unwrap();

    if cfg!(feature = "format") {
        assert!(contents.contains("export type A = { foo: DepA }"));
        assert!(contents.contains("export type B = { foo: DepB }"));
        assert!(contents.contains("export type C = { foo: DepA; bar: DepB; biz: B }"));
    } else {
        assert!(contents.contains(&A::decl()));
        assert!(contents.contains(&B::decl()));
        assert!(contents.contains(&C::decl()));
    }
}
