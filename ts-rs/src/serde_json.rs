use std::collections::HashMap;

use super::{impl_primitives, impl_shadow, TS};

#[derive(TS)]
#[ts(
    crate = "crate",
    rename = "JsonValue",
    untagged,
    export_to = "serde_json/"
)]
pub enum TsJsonValue {
    Number(i32),
    String(String),
    Boolean(bool),
    Array(Vec<TsJsonValue>),
    Object(HashMap<String, TsJsonValue>),
    Null(()),
}

impl_shadow!(as TsJsonValue: impl TS for serde_json::Value);
impl_primitives!(serde_json::Number => "number");
impl_shadow!(as HashMap<K, V>, for serde_json::Map<K, V>, generics: <K: TS, V: TS>);
