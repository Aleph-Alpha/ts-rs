use ts_rs::TS;

#[derive(TS)]
struct Unit;

#[derive(TS)]
struct Unit2 {}

#[derive(TS)]
struct Unit3();

#[test]
fn test() {
    assert_eq!("type Unit = null;", Unit::decl());
    assert_eq!("type Unit2 = null;", Unit2::decl());
    assert_eq!("type Unit3 = null;", Unit3::decl());
}
