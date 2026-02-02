#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};

use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "hashmap/")]
struct Hashes {
    map: HashMap<String, String>,
    set: HashSet<String>,
}

#[test]
fn hashmap() {
    let cfg = Config::from_env();
    assert_eq!(
        Hashes::decl(&cfg),
        "type Hashes = { map: { [key in string]: string }, set: Array<string>, };"
    )
}

struct CustomHasher {}

type CustomHashMap<K, V> = HashMap<K, V, CustomHasher>;
type CustomHashSet<K> = HashSet<K, CustomHasher>;

#[derive(TS)]
#[ts(export, export_to = "hashmap/")]
struct HashesHasher {
    map: CustomHashMap<String, String>,
    set: CustomHashSet<String>,
}

#[test]
fn hashmap_with_custom_hasher() {
    let cfg = Config::from_env();
    assert_eq!(
        HashesHasher::decl(&cfg),
        "type HashesHasher = { map: { [key in string]: string }, set: Array<string>, };"
    )
}

#[derive(TS, Eq, PartialEq, Hash)]
#[ts(export, export_to = "hashmap/")]
struct CustomKey(String);

#[derive(TS)]
#[ts(export, export_to = "hashmap/")]
struct CustomValue;

#[derive(TS)]
#[ts(export, export_to = "hashmap/")]
struct HashMapWithCustomTypes {
    map: HashMap<CustomKey, CustomValue>,
}

#[derive(TS)]
#[ts(export, export_to = "hashmap/")]
struct BTreeMapWithCustomTypes {
    map: BTreeMap<CustomKey, CustomValue>,
}

#[derive(TS)]
#[ts(export, export_to = "hashmap/")]
enum EnumKey {
    Foo,
    Bar,
}

#[test]
fn with_custom_types() {
    let cfg = Config::from_env();
    assert_eq!(
        HashMapWithCustomTypes::inline(&cfg),
        BTreeMapWithCustomTypes::inline(&cfg)
    );
    assert_eq!(
        HashMapWithCustomTypes::decl(&cfg),
        "type HashMapWithCustomTypes = { map: { [key in CustomKey]: CustomValue }, };"
    );
    assert_eq!(
        HashMap::<EnumKey, String>::name(&cfg),
        "{ [key in EnumKey]?: string }"
    );
    assert_eq!(
        HashMap::<EnumKey, String>::inline(&cfg),
        r#"{ [key in "Foo" | "Bar"]?: string }"#
    );
}
