use ts_rs::TS;

#[allow(non_camel_case_types, dead_code)]
#[derive(TS)]
struct r#enum {
    r#type: i32,
    r#use: i32,
    r#struct: i32,
    r#let: i32,
    r#enum: i32,
}

#[test]
fn raw_idents() {
    let out = <r#enum as TS>::decl();
    assert_eq!(
        out,
        "interface enum {\n    type: number,\n    use: number,\n    struct: number,\n    let: number,\n    enum: number,\n}"
    );
}
