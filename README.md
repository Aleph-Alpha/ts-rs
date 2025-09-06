# ts-rs

<h1 align="center" style="padding-top: 0; margin-top: 0;">
<img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/ts-rs/main/logo.png" alt="logo">
<br/>
ts-rs
</h1>
<p align="center">
Generate typescript type declarations from rust types
</p>

<div align="center">
<!-- Github Actions -->
<img src="https://img.shields.io/github/actions/workflow/status/Aleph-Alpha/ts-rs/test.yml?branch=main" alt="actions status" />
<a href="https://crates.io/crates/ts-rs">
<img src="https://img.shields.io/crates/v/ts-rs.svg?style=flat-square"
alt="Crates.io version" />
</a>
<a href="https://docs.rs/ts-rs">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
alt="docs.rs docs" />
</a>
<a href="https://crates.io/crates/ts-rs">
<img src="https://img.shields.io/crates/d/ts-rs.svg?style=flat-square"
alt="Download" />
</a>
</div>

### Why?

When building a web application in rust, data structures have to be shared between backend and frontend.
Using this library, you can easily generate TypeScript bindings to your rust structs & enums so that you can keep your
types in one place.

ts-rs might also come in handy when working with webassembly.

### How?

ts-rs exposes a single trait, `TS`. Using a derive macro, you can implement this interface for your types.
Then, you can use this trait to obtain the TypeScript bindings.
We recommend doing this in your tests.
[See the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/ts-rs/latest/ts_rs/).

### Get started

```toml
[dependencies]
ts-rs = "10.1"
```

```rust
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
}
```

When running `cargo test` or `cargo test export_bindings`, the TypeScript bindings will be exported to the file `bindings/User.ts`
and will contain the following code:

```ts
export type User = { user_id: number; first_name: string; last_name: string };
```

### Features

- generate type declarations from rust structs
- generate union declarations from rust enums
- inline types
- flatten structs/types
- generate necessary imports when exporting to multiple files
- serde compatibility
- generic types
- support for ESM imports

### cargo features

| **Feature**        | **Description**                                                                                                                                    |
| :----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| serde-compat       | **Enabled by default** <br/>See the _"serde compatibility"_ section below for more information.                                                    |
| format             | Enables formatting of the generated TypeScript bindings. <br/>Currently, this unfortunately adds quite a few dependencies.                         |
| no-serde-warnings  | By default, warnings are printed during build if unsupported serde attributes are encountered. <br/>Enabling this feature silences these warnings. |
| serde-json-impl    | Implement `TS` for types from _serde_json_                                                                                                         |
| chrono-impl        | Implement `TS` for types from _chrono_                                                                                                             |
| bigdecimal-impl    | Implement `TS` for types from _bigdecimal_                                                                                                         |
| url-impl           | Implement `TS` for types from _url_                                                                                                                |
| uuid-impl          | Implement `TS` for types from _uuid_                                                                                                               |
| bson-uuid-impl     | Implement `TS` for _bson::oid::ObjectId_ and _bson::uuid_                                                                                          |
| bytes-impl         | Implement `TS` for types from _bytes_                                                                                                              |
| indexmap-impl      | Implement `TS` for types from _indexmap_                                                                                                           |
| ordered-float-impl | Implement `TS` for types from _ordered_float_                                                                                                      |
| heapless-impl      | Implement `TS` for types from _heapless_                                                                                                           |
| semver-impl        | Implement `TS` for types from _semver_                                                                                                             |
| smol_str-impl      | Implement `TS` for types from _smol_str_                                                                                                           |
| tokio-impl         | Implement `TS` for types from _tokio_                                                                                                              |
| no-big-int         | Always bind primitive integers to raw javascript numbers even when this could lead to precision loss                                               |

<br/>

If there's a type you're dealing with which doesn't implement `TS`, use either
`#[ts(as = "..")]` or `#[ts(type = "..")]`, or open a PR.

### `serde` compatability

With the `serde-compat` feature (enabled by default), serde attributes can be parsed for enums and structs.
Supported serde attributes:

- `rename`
- `rename-all`
- `rename-all-fields`
- `tag`
- `content`
- `untagged`
- `skip`
- `skip_serializing`
- `skip_serializing_if`
- `flatten`
- `default`

Note: `skip_serializing` and `skip_serializing_if` only have an effect when used together with
`#[serde(default)]`.

Note: `skip_deserializing` is ignored. If you wish to exclude a field
from the generated type, but cannot use `#[serde(skip)]`, use `#[ts(skip)]` instead.

When ts-rs encounters an unsupported serde attribute, a warning is emitted, unless the feature `no-serde-warnings` is enabled.

### Contributing

Contributions are always welcome!
Feel free to open an issue, discuss using GitHub discussions or open a PR.
[See CONTRIBUTING.md](https://github.com/Aleph-Alpha/ts-rs/blob/main/CONTRIBUTING.md)

### MSRV

The Minimum Supported Rust Version for this crate is 1.78.0

License: MIT
