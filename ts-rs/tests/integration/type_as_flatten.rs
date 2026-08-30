#![allow(dead_code)]

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "type_as_flatten/")]
struct Substitute {
    alpha: String,
    beta: i32,
}

#[derive(TS)]
#[ts(as = "Substitute")]
#[ts(export, export_to = "type_as_flatten/")]
struct TypeWithAs {
    _internal: String,
}

#[derive(TS)]
#[ts(export, export_to = "type_as_flatten/")]
struct OuterWithTypeAs {
    name: String,
    #[ts(flatten)]
    inner: TypeWithAs,
}

#[test]
fn test_type_as_flatten() {
    let cfg = Config::from_env();
    assert_eq!(TypeWithAs::inline(&cfg), "{ alpha: string, beta: number, }");
    assert_eq!(
        OuterWithTypeAs::inline(&cfg),
        "{ name: string, alpha: string, beta: number, }"
    );
}
