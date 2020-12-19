use ts_rs::TS;

#[derive(TS)]
struct Unit;

#[test]
fn test() {
    assert_eq!("export type Unit = null;", Unit::decl())
}
