use ts_rs::TS;

#[test]
fn list() {
    #[derive(TS)]
    struct List {
        #[allow(dead_code)]
        data: Option<Vec<u32>>,
    }

    assert_eq!(
        List::decl(),
        "interface List {\n    data: Array<number> | null,\n}"
    );
}
