#![allow(dead_code)]

use serde::Deserialize;
use ts_rs::TS;

#[derive(TS, Deserialize)]
#[serde(untagged)]
enum TestUntagged {
    A,   // serde_json -> `null`
    B(), // serde_json -> `[]`
    C {
        #[serde(skip)]
        val: i32,
    }, // serde_json -> `{}`
}

#[derive(TS, Deserialize)]
enum TestExternally {
    A,   // serde_json -> `"A"`
    B(), // serde_json -> `{"B":[]}`
    C {
        #[serde(skip)]
        val: i32,
    }, // serde_json -> `{"C":{}}`
}

#[derive(TS, Deserialize)]
#[serde(tag = "type", content = "content")]
enum TestAdjacently {
    A,   // serde_json -> `{"type":"A"}`
    B(), // serde_json -> `{"type":"B","content":[]}`
    C {
        #[serde(skip)]
        val: i32,
    }, // serde_json -> `{"type":"C","content":{}}`
}

#[derive(TS, Deserialize)]
#[serde(tag = "type")]
enum TestInternally {
    A, // serde_json -> `{"type":"A"}`
    B, // serde_json -> `{"type":"B"}`
    C {
        #[serde(skip)]
        val: i32,
    }, // serde_json -> `{"type":"C"}`
}

#[cfg(feature = "serde-compat")]
#[test]
fn test() {
    assert_eq!(
        TestUntagged::decl(),
        r#"type TestUntagged = null | never[] | {  };"#
    );

    assert_eq!(
        TestExternally::decl(),
        r#"type TestExternally = "A" | { "B": never[] } | { "C": {  } };"#
    );

    assert_eq!(
        TestAdjacently::decl(),
        r#"type TestAdjacently = { "type": "A" } | { "type": "B", "content": never[] } | { "type": "C", "content": {  } };"#
    );

    assert_eq!(
        TestInternally::decl(),
        r#"type TestInternally = { "type": "A" } | { "type": "B" } | { "type": "C",  };"#
    );
}
