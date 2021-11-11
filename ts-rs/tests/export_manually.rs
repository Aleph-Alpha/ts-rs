#![allow(dead_code)]

use std::{concat, fs};

use ts_rs::TS;

#[derive(TS)]
#[ts(export_to = "export_here_test.ts")]
struct User {
    name: String,
    age: i32,
    active: bool,
}

#[test]
fn export_manually() {
    User::export().unwrap();

    let expected_content = concat!(
        "export interface User {\n",
        "  name: string;\n",
        "  age: number;\n",
        "  active: boolean;\n",
        "}\n"
    );

    let actual_content = fs::read_to_string("export_here_test.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}
