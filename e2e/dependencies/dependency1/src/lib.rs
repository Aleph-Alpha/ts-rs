use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub struct LibraryType {
    pub a: i32
}

#[test]
fn env_set() {
    assert_ne!(env!("TS_RS_EXPORT_DIR"), "");
}
