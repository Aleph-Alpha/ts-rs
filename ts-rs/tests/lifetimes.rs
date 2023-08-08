use ts_rs::TS;

#[test]
fn contains_borrow() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct S<'a> {
        s: &'a str,
    }

    assert_eq!(S::decl(), "interface S { s: string, }")
}
