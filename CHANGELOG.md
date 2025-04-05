# master
### Breaking
- Changed return type of `TS::output_path()` from `Option<&'static Path>` to `Option<PathBuf>`.  
  This will only break your code if you manually implement `TS` or directly interact with the `TS` trait.
- Added `OptionInnerType` associated type to the `TS` trait. If you manually implement `TS`, you must set this associated type to `Self` in all of your implementations.
- Raised MSRV to `1.78.0` due to use of `#[diagnostic::on_unimplemented]` and `let ... else { ... }`

### Features
- The `#[ts(rename)]` attribute on structs, enums and variants now accepts any expression.  
  This makes it possible to, for example, rename a struct to the name of a module it is contained in using `#[ts(rename = module_path!().rsplit_once("::").unwrap().1)]`
- The `#[ts(export_to)]` attribute on structs and enums now accepts any expression.
- Added `#[ts(optional_fields)]` and `#[ts(optional_fields = nullable)]` attribute to structs, this attribute is equivalent to using the corresponding `#[ts(optional)]` or `#[ts(optional = nullable)]` on every field of the struct. ([#366](https://github.com/Aleph-Alpha/ts-rs/pull/366))

### Fixes
- Fix `#[ts(optional)]` error when using a type alias for `Option` or fully qualifying it as `core::option::Option` ([#366](https://github.com/Aleph-Alpha/ts-rs/pull/366))
- Fix missing import statements when using `#[ts(as = "...")]` at the top level of a struct/enum ([#385](https://github.com/Aleph-Alpha/ts-rs/pull/385))

# 10.1.0
### Features
- Add support for synchronization primitives from `tokio` (feature `tokio-impl`)
### Fixes
- Fix incorrect behavior of the tag attribute for structs without any fields declared with braces
- Fix representation of `serde_json::Value`

# 10.0.0
### Breaking
- Change how `HashMap<K, V>` is represented in TypeScript. The resulting bindings (`{ [key in K]?: V }` instead of `{ [key: K]: V }`) are more accurate and flexible.

### Features

- Allow multile types to have the same `#[ts(export_to = "...")]` attribute and be exported to the same file ([#316](https://github.com/Aleph-Alpha/ts-rs/pull/316))
- The `bson-uuid-impl` feature now supports `bson::oid::ObjectId` as well ([#340](https://github.com/Aleph-Alpha/ts-rs/pull/340))
- Add support for types from `smol_str` behind cargo feature `smol_str-impl` ([#350](https://github.com/Aleph-Alpha/ts-rs/pull/350))
- Support `#[ts(as = "...")]` and `#[ts(type = "...")]` on enum variants ([#384](https://github.com/Aleph-Alpha/ts-rs/pull/384))

### Fixes

- Properly handle block doc comments ([#342](https://github.com/Aleph-Alpha/ts-rs/pull/342))
- Fix error in internally tagged enums with flattened fields ([#344](https://github.com/Aleph-Alpha/ts-rs/pull/344))
- Always use forward slash on import paths ([#346](https://github.com/Aleph-Alpha/ts-rs/pull/346))

# 9.0.1
### Fixes
- Allow using `#[ts(flatten)]` on fields using generic parameters ([#336](https://github.com/Aleph-Alpha/ts-rs/pull/336))


# 9.0.0

### Breaking

- `#[serde(with = "...")]` requires the use of `#[ts(as = "...")]` or `#[ts(type = "...")]` ([#280](https://github.com/Aleph-Alpha/ts-rs/pull/280))
- Fix incompatibility with serde for `snake_case`, `kebab-case` and `SCREAMING_SNAKE_CASE` ([#298](https://github.com/Aleph-Alpha/ts-rs/pull/298))
- `#[ts(rename_all = "...")]` no longer accepts variations in the string's casing, dashes and underscores to make behavior consistent with serde ([#298](https://github.com/Aleph-Alpha/ts-rs/pull/298))
- Remove `TypeList`, and replace `TS::dependency_types`/`TS::generics` with `TS::visit_dependencies`/`TS::visit_generics`.
  This finally resolves "overflow evaluating the requirement", "reached the recursion limit" errors.
  Also, compile times should benefit. This is a technically breaking change for those interacting with the `TS` trait
  directly. For those just using `#[derive(TS)]` and `#[ts(...)]`, nothing changes!

### Features

- Add support for `#[ts(type = "..")]` directly on structs and enums ([#286](https://github.com/Aleph-Alpha/ts-rs/pull/286))
- Add support for `#[ts(as = "..")]` directly on structs and enums ([#288](https://github.com/Aleph-Alpha/ts-rs/pull/288))
- Add support for `#[ts(rename_all = "SCREAMING-KEBAB-CASE")]` ([#298](https://github.com/Aleph-Alpha/ts-rs/pull/298))
- Support `_` in `#[ts(type = "..")]` to refer to the type of the field ([#299](https://github.com/Aleph-Alpha/ts-rs/pull/299))

### Fixes

- Fix `#[ts(rename_all_fields = "...")]` on enums containing tuple or unit variants ([#287](https://github.com/Aleph-Alpha/ts-rs/pull/287))
- Fix "overflow evaluating the requirement" and "reached the recursion limit" errors in some cases ([#293](https://github.com/Aleph-Alpha/ts-rs/pull/293))
- Fix ambiguity causing "multiple applicable items in scope" errors in some cases ([#309](https://github.com/Aleph-Alpha/ts-rs/pull/309))
- Fix issues with absolute `TS_RS_EXPORT_DIR` paths ([#323](https://github.com/Aleph-Alpha/ts-rs/pull/323))
- Add newlines to the end of exported files ([#321](https://github.com/Aleph-Alpha/ts-rs/pull/321))

# 8.1.0

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
