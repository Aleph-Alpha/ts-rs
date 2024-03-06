# ts-rs

<h1 align="center" style="padding-top: 0; margin-top: 0;">
<img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/ts-rs/main/logo.png" alt="logo">
<br/>
ts-rs
</h1>
<p align="center">
generate typescript interface/type declarations from rust types
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

### why?
When building a web application in rust, data structures have to be shared between backend and frontend.
Using this library, you can easily generate TypeScript bindings to your rust structs & enums so that you can keep your
types in one place.

ts-rs might also come in handy when working with webassembly.

### how?
ts-rs exposes a single trait, `TS`. Using a derive macro, you can implement this interface for your types.
Then, you can use this trait to obtain the TypeScript bindings.
We recommend doing this in your tests.
[See the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/ts-rs/latest/ts_rs/).

### get started
```toml
[dependencies]
ts-rs = "7.1"
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
When running `cargo test`, the TypeScript bindings will be exported to the file `bindings/User.ts`.

### features
- generate interface declarations from rust structs
- generate union declarations from rust enums
- inline types
- flatten structs/interfaces
- generate necessary imports when exporting to multiple files
- serde compatibility
- generic types
- support for ESM imports

### cargo features
- **serde-compat** (default)

  Enable serde compatibility. See below for more info.
  <br/><br/>

- **format**

  When enabled, the generated typescript will be formatted.
  Currently, this sadly adds quite a few dependencies.
  <br/><br/>

- **no-serde-warnings**

  When `serde-compat` is enabled, warnings are printed during build if unsupported serde
  attributes are encountered.
  Enabling this feature silences these warnings.
  <br/><br/>

- **import-esm**

  `import` statements in the generated file will have the `.js` extension in the end of
  the path to conform to the ES Modules spec. (e.g.: `import { MyStruct } from "./my_struct.js"`)
  <br/><br/>

- **chrono-impl**

  Implement `TS` for types from chrono
  <br/><br/>

- **bigdecimal-impl**

  Implement `TS` for types from bigdecimal
  <br/><br/>

- **url-impl**

  Implement `TS` for types from url
  <br/><br/>

- **uuid-impl**

  Implement `TS` for types from uuid
  <br/><br/>

- **bson-uuid-impl**

  Implement `TS` for types from bson
  <br/><br/>

- **bytes-impl**

  Implement `TS` for types from bytes
  <br/><br/>

- **indexmap-impl**

  Implement `TS` for `IndexMap` and `IndexSet` from indexmap
  <br/><br/>

- **ordered-float-impl**

  Implement `TS` for `OrderedFloat` from ordered_float
  <br/><br/>

- **heapless-impl**

  Implement `TS` for `Vec` from heapless
  <br/><br/>

- **semver-impl**

  Implement `TS` for `Version` from semver
  <br/><br/>


If there's a type you're dealing with which doesn't implement `TS`, use either
`#[ts(as = ".."`)] or `#[ts(type = "..")]`, or open a PR.

### serde compatability
With the `serde-compat` feature (enabled by default), serde attributes can be parsed for enums and structs.
Supported serde attributes:
- `rename`
- `rename-all`
- `rename-all-fields`
- `tag`
- `content`
- `untagged`
- `skip`
- `flatten`
- `default`

Note: `skip_serializing` and `skip_deserializing` are ignored. If you wish to exclude a field
from the generated type, but cannot use `#[serde(skip)]`, use `#[ts(skip)]` instead.

When ts-rs encounters an unsupported serde attribute, a warning is emitted, unless the feature `no-serde-warnings` is enabled.

### contributing
Contributions are always welcome!
Feel free to open an issue, discuss using GitHub discussions or open a PR.
[See CONTRIBUTING.md](https://github.com/Aleph-Alpha/ts-rs/blob/main/CONTRIBUTING.md)

### todo
- [x] serde compatibility layer
- [x] documentation
- [x] use typescript types across files
- [x] more enum representations
- [x] generics
- [x] don't require `'static`

License: MIT
