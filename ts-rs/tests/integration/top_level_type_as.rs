use ts_rs::TS;

#[derive(TS)]
#[ts(as = "T")]
pub enum UntaggedEnum<T: TS> {
    Left(T),
    Right(T),
}

#[test]
pub fn top_level_type_as_enum() {
    assert_eq!(UntaggedEnum::<String>::inline(), r#"string"#)
}

#[derive(TS)]
#[ts(as = "T")]
pub struct Wrapper<T: TS>(T);

#[test]
pub fn top_level_type_as_struct() {
    assert_eq!(Wrapper::<String>::inline(), r#"string"#)
}
