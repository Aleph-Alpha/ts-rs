#![cfg(feature = "indexmap-impl")]

use indexmap::{IndexMap, IndexSet};
use ts_rs::TS;

#[test]
fn indexmap() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct Indexes {
        map: IndexMap<String, String>,
        set: IndexSet<String>,
    }

    assert_eq!(
        Indexes::decl(),
        "interface Indexes { map: Record<string, string>, set: Array<string>, }"
    )
}
