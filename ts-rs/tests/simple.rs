#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct Simple {
    a: i32,
    b: String,
    c: (i32, String, i32),
    d: Vec<String>,
    e: Option<String>,
}

#[test]
fn test_def() {
    assert_eq!(
        Simple::inline(0),
        "\
{
    a: number,
    b: string,
    c: [number, string, number],
    d: Array<string>,
    e: string | null,
}"
    )
}

#[test]
fn test_indented() {
    assert_eq!(
        Simple::inline(1),
        "\
{
        a: number,
        b: string,
        c: [number, string, number],
        d: Array<string>,
        e: string | null,
    }"
    )
}
