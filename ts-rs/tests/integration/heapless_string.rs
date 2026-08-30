#![cfg(feature = "heapless-impl")]
// Import `heapless::String` directly so that the bare identifier `String` is
// shadowed within this module. This mirrors idiomatic heapless usage and is a
// regression test: the `TS` derive macro must emit `::std::string::String` in
// the code it generates, otherwise it resolves to `heapless::String` (a type
// alias requiring a generic argument) and fails to compile.
use heapless::{String, Vec};
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "heapless_string/")]
struct ImStackAllocated {
    name: String<32>,
    nested: Vec<String<8>, 4>,
}

#[test]
fn heapless_string() {
    let cfg = Config::from_env();
    assert_eq!(
        ImStackAllocated::decl(&cfg),
        "type ImStackAllocated = { name: string, nested: Array<string>, };"
    )
}
