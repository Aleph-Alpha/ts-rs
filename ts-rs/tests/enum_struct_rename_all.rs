use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export)]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
pub enum TaskStatus {
  #[serde(rename_all = "camelCase")]
  Running { started_time: String },
  #[serde(rename_all = "camelCase")]
  Terminated {
    status: i32,
    stdout: String,
    stderr: String,
  },
}

#[test]
pub fn enum_struct_rename_all() {
    assert_eq!(TaskStatus::inline(), r#"{ "running": { startedTime: string, } } | { "terminated": { status: number, stdout: string, stderr: string, } }"#)
}
