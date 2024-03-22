# master

### Breaking

### Features

- Add `#[ts(crate = "..")]` to allow usage of `#[derive(TS)]` from other proc-macro crates ([#274](https://github.com/Aleph-Alpha/ts-rs/pull/274))
- Add support types from `serde_json` behind cargo feature `serde-json-impl` ([#276](https://github.com/Aleph-Alpha/ts-rs/pull/276))

### Fixes

- Macro expansion for types with generic parameters now works without the `TS` trait in scope ([#281](https://github.com/Aleph-Alpha/ts-rs/pull/281))
- Fix enum flattening a struct that contains a flattened enum ([#282](https://github.com/Aleph-Alpha/ts-rs/pull/282))

# v8.0.0

### Breaking

- Export types as `type` instead of `ìnterface` ([#203](https://github.com/Aleph-Alpha/ts-rs/pull/203))
- Automatically export all dependencies when using `#[ts(export)]`, add `TS::dependency_types()` ([#221](https://github.com/Aleph-Alpha/ts-rs/pull/221))
- Remove support for "skip_serializing", "skip_serializing_if" and "skip_deserializing". ([#204](https://github.com/Aleph-Alpha/ts-rs/pull/204))
  - Initially supporting these by skipping a field was a mistake. If a user wishes to skip a field, they can still
    annotate it with `#[ts(skip)]`
- Added `TS::dependency_types()` ([#221](https://github.com/Aleph-Alpha/ts-rs/pull/221))
- Added `TS::generics()` ([#241](https://github.com/Aleph-Alpha/ts-rs/pull/241))
- Added `TS::WithoutGenerics` ([#241](https://github.com/Aleph-Alpha/ts-rs/pull/241))
- Removed `TS::transparent()` ([#243](https://github.com/Aleph-Alpha/ts-rs/pull/243))
- Handling of output paths ([#247](https://github.com/Aleph-Alpha/ts-rs/pull/247), [#250](https://github.com/Aleph-Alpha/ts-rs/pull/250), [#256](https://github.com/Aleph-Alpha/ts-rs/pull/256))
  - All paths specified using `#[ts(export_to = "...")]` are now relative to `TS_RS_EXPORT_DIR`, which defaults to `./bindings/`
- Replace `TS::export` with `TS::export`, `TS::export_all` and `TS::export_to_all` ([#263](https://github.com/Aleph-Alpha/ts-rs/pull/263))

### Features

- Implement `#[ts(as = "..")]` ([#174](https://github.com/Aleph-Alpha/ts-rs/pull/174))
- For small arrays, generate tuples instead of `Array<T>` ([#209](https://github.com/Aleph-Alpha/ts-rs/pull/209))
- Implement `#[ts(optional = nullable)]` ([#213](https://github.com/Aleph-Alpha/ts-rs/pull/213))
- Allow inlining of fields with generic types ([#212](https://github.com/Aleph-Alpha/ts-rs/pull/212), [#215](https://github.com/Aleph-Alpha/ts-rs/pull/215), [#216](https://github.com/Aleph-Alpha/ts-rs/pull/216))
- Allow flattening enum fields ([#206](https://github.com/Aleph-Alpha/ts-rs/pull/206))
- Add `semver-impl` cargo feature with support for the _semver_ crate ([#176](https://github.com/Aleph-Alpha/ts-rs/pull/176))
- Support `HashMap` with custom hashers ([#173](https://github.com/Aleph-Alpha/ts-rs/pull/173))
- Add `import-esm` cargo feature to import files with a `.js` extension ([#192](https://github.com/Aleph-Alpha/ts-rs/pull/192))
- Implement `#[ts(...)]` equivalents for `#[serde(tag = "...")]`, `#[serde(tag = "...", content = "...")]` and `#[serde(untagged)]` ([#227](https://github.com/Aleph-Alpha/ts-rs/pull/227))
- Support `#[serde(untagged)]` on individual enum variants ([#226](https://github.com/Aleph-Alpha/ts-rs/pull/226))
- Support for `#[serde(rename_all_fields = "...")]` ([#225](https://github.com/Aleph-Alpha/ts-rs/pull/225))
- Export Rust doc comments/attributes on structs/enums as TSDoc strings ([#187](https://github.com/Aleph-Alpha/ts-rs/pull/187))
- `Result`, `Option`, `HashMap` and `Vec` had their implementations of `TS` changed ([#241](https://github.com/Aleph-Alpha/ts-rs/pull/241))
- Implement `#[ts(...)]` equivalent for `#[serde(tag = "...")]` being used on a struct with named fields ([#244](https://github.com/Aleph-Alpha/ts-rs/pull/244))
- Implement `#[ts(concrete(..))]` to specify a concrete type for a generic parameter ([#264](https://github.com/Aleph-Alpha/ts-rs/pull/264))

### Fixes

- Fix `#[ts(skip)]` and `#[serde(skip)]` in variants of adjacently or internally tagged enums ([#231](https://github.com/Aleph-Alpha/ts-rs/pull/231))
- `rename_all` with `camelCase` produces wrong names if fields were already in camelCase ([#198](https://github.com/Aleph-Alpha/ts-rs/pull/198))
- Improve support for references ([#199](https://github.com/Aleph-Alpha/ts-rs/pull/199))
- Generic type aliases generate correctly ([#233](https://github.com/Aleph-Alpha/ts-rs/pull/233))
- Improve compiler errors ([#257](https://github.com/Aleph-Alpha/ts-rs/pull/257))
- Update dependencies ([#255](https://github.com/Aleph-Alpha/ts-rs/pull/255))
