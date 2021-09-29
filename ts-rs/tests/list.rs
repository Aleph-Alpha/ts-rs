use ts_rs::TS;

#[test]
fn list() {
    #[derive(TS)]
    struct List {
        data: Option<Vec<u32>>
    }

    println!("{:?}", List::decl());
    assert_eq!(List::decl(), "");
}