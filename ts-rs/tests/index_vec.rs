#![cfg(feature = "index_vec-impl")]

use index_vec::{define_index_type, IndexVec};
use ts_rs::TS;

#[test]
fn index_vec() {
    define_index_type! {
        pub struct MyIdx = u32;
    }

    #[derive(TS)]
    #[allow(dead_code)]
    struct Indexes {
        vec: IndexVec<MyIdx, String>,
    }

    assert_eq!(Indexes::decl(), "type Indexes = { vec: Array<string>, }")
}
