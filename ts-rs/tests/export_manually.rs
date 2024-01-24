#![allow(dead_code)]

use std::{concat, fs};

use ts_rs::TS;

#[derive(TS)]
#[ts(export_to = "tests-out/export_here_test.ts")]
struct User {
    name: String,
    age: i32,
    active: bool,
}

#[derive(TS)]
#[ts(export_to = "tests-out/export_here_dir_test/")]
struct UserDir {
    name: String,
    age: i32,
    active: bool,
}

#[test]
fn export_manually() {
    User::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n",
            "\nexport type User = { name: string; age: number; active: boolean };\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n",
            "\nexport type User = { name: string, age: number, active: boolean, }"
        )
    };

    let actual_content = fs::read_to_string("tests-out/export_here_test.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_manually_dir() {
    UserDir::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n",
            "\nexport type UserDir = { name: string; age: number; active: boolean };\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n",
            "\nexport type UserDir = { name: string, age: number, active: boolean, }"
        )
    };

    let actual_content = fs::read_to_string("tests-out/export_here_dir_test/UserDir.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}
