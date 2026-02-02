use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "top_level_type_override/")]
#[ts(type = "string")]
#[non_exhaustive]
pub enum IncompleteEnum {
    Foo,
    Bar,
    Baz,
    // more
}

#[test]
pub fn top_level_type_override_enum() {
    let cfg = Config::from_env();
    assert_eq!(IncompleteEnum::inline(&cfg), r#"string"#)
}

#[derive(TS)]
#[ts(export, export_to = "top_level_type_override/")]
#[ts(type = "string")]
pub struct DataUrl {
    pub mime: String,
    pub contents: Vec<u8>,
}

#[test]
pub fn top_level_type_override_struct() {
    let cfg = Config::from_env();
    assert_eq!(DataUrl::inline(&cfg), r#"string"#)
}
