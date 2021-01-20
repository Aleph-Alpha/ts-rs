#![allow(dead_code)]

use ts_rs::TS;
use serde::{Deserialize};


#[derive(TS, Deserialize)]
#[serde(tag="kind")]
enum SimpleEnum {
    A,
    B,
}

#[test]
fn test_serde_enum() {
    assert_eq!(
        SimpleEnum::decl(),
        r#"export type SimpleEnum = {kind: "A"} | {kind: "B"};"#
    )
}
