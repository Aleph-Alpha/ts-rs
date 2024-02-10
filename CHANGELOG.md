# master

### Breaking 
- Export types as `type` instead of `ìnterface` ([#203](https://github.com/Aleph-Alpha/ts-rs/pull/203))
- Automatically export all dependencies when using `#[ts(export)]`, add `TS::dependency_types()` ([#221](https://github.com/Aleph-Alpha/ts-rs/pull/221))
- Remove support for "skip_serializing", "skip_serializing_if" and "skip_deserializing". ([#204](https://github.com/Aleph-Alpha/ts-rs/pull/204))
    - Initially supporting these by skipping a field was a mistake. If a user wishes to skip a field, they can still
      annotate it with `#[ts(skip)]`
 
### Features
- Implement `#[ts(as = "..")]` ([#174](https://github.com/Aleph-Alpha/ts-rs/pull/174))
- For small arrays, generate tuples instead of `Array<T>` ([#209](https://github.com/Aleph-Alpha/ts-rs/pull/209))
- Implement `#[ts(optional = nullable)]` ([#213](https://github.com/Aleph-Alpha/ts-rs/pull/213))
- Allow inlining of fields with generic types ([#212](https://github.com/Aleph-Alpha/ts-rs/pull/212), [#215](https://github.com/Aleph-Alpha/ts-rs/pull/215), [#216](https://github.com/Aleph-Alpha/ts-rs/pull/216))
- Allow flattening enum fields ([#206](https://github.com/Aleph-Alpha/ts-rs/pull/206))
- Add `semver-impl` cargo feature with support for the *semver* crate ([#176](https://github.com/Aleph-Alpha/ts-rs/pull/176))
- Support `HashMap` with custom hashers ([#173](https://github.com/Aleph-Alpha/ts-rs/pull/173))
- Add `import-esm` cargo feature to import files with a `.js` extension ([#192](https://github.com/Aleph-Alpha/ts-rs/pull/192))
- Implement `#[ts(...)]` equivalents for `#[serde(tag = "...")]`, `#[serde(tag = "...", content = "...")]` and `#[serde(untagged)]` ([#227](https://github.com/Aleph-Alpha/ts-rs/pull/227))
- Support `#[serde(untagged)]` on individual enum variants ([#226](https://github.com/Aleph-Alpha/ts-rs/pull/226))
- Support for `#[serde(rename_all_fields = "...")]` ([#225](https://github.com/Aleph-Alpha/ts-rs/pull/225))
- Export Rust doc comments/attributes on structs/enums as TSDoc strings ([#187](https://github.com/Aleph-Alpha/ts-rs/pull/187))

### Fixes
- fix `#[ts(skip)]` and `#[serde(skip)]` in variants of adjacently or internally tagged enums ([#231](https://github.com/Aleph-Alpha/ts-rs/pull/231))
- `rename_all` with `camelCase` produces wrong names if fields were already in camelCase ([#198](https://github.com/Aleph-Alpha/ts-rs/pull/198))
- Improve support for references ([#199](https://github.com/Aleph-Alpha/ts-rs/pull/199))

