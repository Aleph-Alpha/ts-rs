use std::collections::HashMap;

use super::{impl_primitives, impl_shadow, TS};

#[derive(TS)]
#[ts(
    crate = "crate",
    rename = "Value",
    untagged,
    export_to = "serde_json/"
)]
pub enum TsJsonValue {
    Number(i32),
    String(String),
    Array(Vec<TsJsonValue>),
    Object(HashMap<String, TsJsonValue>),
}

impl_shadow!(as TsJsonValue: impl TS for serde_json::Value);
impl_primitives!(serde_json::Number => "number");
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for serde_json::Map<K, V>);
