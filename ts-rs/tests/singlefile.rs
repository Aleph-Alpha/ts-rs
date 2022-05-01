#![allow(dead_code)]

use ts_rs::{SingleFileExporter, TS};

#[derive(TS)]
struct Alpha {
    b: Beta,
}

#[derive(TS)]
struct Beta {
    x: bool,
}

#[test]
fn test_singlefile() {
    let out = SingleFileExporter::new(false)
        .and::<Alpha>()
        .and::<Beta>()
        .finish()
        .unwrap();

    assert_eq!(
        out,
        r#"export interface Alpha { b: Beta, }

export interface Beta { x: boolean, }
"#
    );
}
