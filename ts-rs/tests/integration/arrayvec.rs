#![cfg(feature = "arrayvec-impl")]
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "arrayvec/")]
struct ImStackAllocated {
    smol_vec: arrayvec::ArrayVec<f32, 16>,
    name: arrayvec::ArrayString<32>,
    nested: arrayvec::ArrayVec<arrayvec::ArrayVec<arrayvec::ArrayString<8>, 2>, 4>,
}

#[test]
fn arrayvec() {
    let cfg = Config::from_env();
    assert_eq!(ImStackAllocated::decl(&cfg), "type ImStackAllocated = { smol_vec: Array<number>, name: string, nested: Array<Array<string>>, };")
}
