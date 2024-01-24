use ts_rs::TS;

// serde_json serializes this to `null`, so it's TS type is `null` as well.
#[derive(TS)]
struct Unit;

// serde_json serializes this to `{}`.
// The TS type best describing an empty object is `{ }`.
#[derive(TS)]
struct Unit2 {}

// serde_json serializes this to `[]`.
// The TS type best describing an empty array is `never[]`.
#[derive(TS)]
struct Unit3();

// serde_json serializes this to `null`, so it's TS type is `null` as well.
#[derive(TS)]
struct Unit4(());

#[test]
fn test() {
    assert_eq!("type Unit = null;", Unit::decl());
    assert_eq!("type Unit2 = { };", Unit2::decl());
    assert_eq!("type Unit3 = never[];", Unit3::decl());
    assert_eq!("type Unit4 = null;", Unit4::decl());
}
