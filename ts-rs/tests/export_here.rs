#![allow(dead_code)]

use std::{concat, env, fs};

use ts_rs::TS;

#[derive(TS)]
struct User {
    name: String,
    age: i32,
    active: bool,
}

#[test]
fn test_export_here() {
    let dir = env::temp_dir();
    let file_path = dir.join("User.ts");
    ts_rs::export_here!(User => file_path.to_str().unwrap());

    let expected_content = concat!(
        "export interface User {\n",
        "  name: string;\n",
        "  age: number;\n",
        "  active: boolean;\n",
        "}"
    );

    let actual_content = fs::read_to_string(file_path).unwrap();

    assert_eq!(actual_content, expected_content);
}
