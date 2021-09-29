use ts_rs::TS;

#[test]
fn list() {
    #[derive(TS)]
    struct List {
        data: Option<Vec<u32>>
    }

    assert_eq!(List::decl(&[]), "interface List {\n    data: Array<number> | null,\n}");
}