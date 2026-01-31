use serde::Serialize;
use ts_rs::{Config, TS};

#[derive(TS, Serialize)]
#[ts(export, export_to = "issue_80/")]
pub enum SomeTypeList {
    Value1 {
        #[serde(skip)]
        #[ts(skip)]
        skip_this: String,
    },
    Value2,
}

#[test]
fn issue_80() {
    let cfg = Config::from_env();
    let ty = SomeTypeList::inline(&cfg);
    assert_eq!(ty, r#"{ "Value1": {  } } | "Value2""#);
}
