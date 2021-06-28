#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
struct Optional {
    #[ts(optional)]
    a: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<String>,
}

#[test]
fn test() {
    assert_eq!(
        Optional::inline(0),
        "\
{
    a?: number,
    b?: string,
}"
    )
}
