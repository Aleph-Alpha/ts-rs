use std::collections::{HashMap, HashSet};

use ts_rs::TS;

#[test]
fn hashmap() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct Hashes {
        map: HashMap<String, String>,
        set: HashSet<String>,
    }

    assert_eq!(
        Hashes::decl(),
        "type Hashes = { map: Record<string, string>, set: Array<string>, }"
    )
}

#[test]
fn hashmap_with_custom_hasher() {
    struct CustomHasher {}

    type CustomHashMap<K, V> = HashMap<K, V, CustomHasher>;
    type CustomHashSet<K> = HashSet<K, CustomHasher>;

    #[derive(TS)]
    #[allow(dead_code)]
    struct Hashes {
        map: CustomHashMap<String, String>,
        set: CustomHashSet<String>,
    }

    assert_eq!(
        Hashes::decl(),
        "type Hashes = { map: Record<string, string>, set: Array<string>, }"
    )
}
