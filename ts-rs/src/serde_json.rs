use std::{collections::HashMap, path::Path};

use super::{impl_primitives, impl_shadow, typelist::TypeList, TS};

// Manually implement `TS` for `Value`.
// Defining an untagged enum and deriving `TS` doesn't work for the following reasons:
// - `#[derive(TS)]` doesn't work in this crate, since the macro generates `::ts_rs::TS`
// - `Value` is self-referential, and in this case (an untagged enum), tsc doesn't compile
//   `type JsonValue = ... | Record<string, JsonValue>`, but `{ [x: string]: JsonValue }` works.
impl TS for serde_json::Value {
    type WithoutGenerics = Self;

    fn ident() -> String {
        "JsonValue".to_owned()
    }

    fn decl() -> String {
        Self::decl_concrete()
    }

    fn decl_concrete() -> String {
        format!("type {} = {};", "JsonValue", Self::inline())
    }

    fn name() -> String {
        Self::ident()
    }

    fn inline() -> String {
        let name = Self::name();
        format!("number | string | Array<{name}> | {{ [key: string]: JsonValue }}")
    }

    fn inline_flattened() -> String {
        Self::inline()
    }

    fn dependency_types() -> impl TypeList {
        ().push::<Self>()
    }

    fn output_path() -> Option<&'static Path> {
        Some(Path::new("serde_json/Value.ts"))
    }
}

impl_primitives!(serde_json::Number => "number");
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for serde_json::Map<K, V>);
