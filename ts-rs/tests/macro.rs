use ts_rs::TS;

#[derive(TS)]
struct T {
    foo: String,
}

#[test]
fn test_indented() {
    println!("hello test");

    ts_rs::export_here!(T => "a.ts");
    assert_eq!(1, 1);
}
