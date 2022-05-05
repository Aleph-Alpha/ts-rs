#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
struct DocString {
    #[ts(doc_string="@mydoc")]
    a: i32,
    #[ts(doc_string="@mydoc2")]
    b: String,
}

#[test]
fn test() {
    assert_eq!(DocString::inline(), "{ /** @mydoc */ a: number, /** @mydoc2 */ b: string, }");
}
