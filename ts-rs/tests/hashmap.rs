#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "tests-out/hashmap/")]
struct Hashes {
    map: HashMap<String, String>,
    set: HashSet<String>,
}

#[test]
fn hashmap() {
    assert_eq!(
        Hashes::decl(),
        "type Hashes = { map: Record<string, string>, set: Array<string>, };"
    )
}

struct CustomHasher {}

type CustomHashMap<K, V> = HashMap<K, V, CustomHasher>;
type CustomHashSet<K> = HashSet<K, CustomHasher>;

#[derive(TS)]
#[ts(export, export_to = "tests-out/hashmap/")]
struct HashesHasher {
    map: CustomHashMap<String, String>,
    set: CustomHashSet<String>,
}

#[test]
fn hashmap_with_custom_hasher() {
    assert_eq!(
        HashesHasher::decl(),
        "type HashesHasher = { map: Record<string, string>, set: Array<string>, };"
    )
}
