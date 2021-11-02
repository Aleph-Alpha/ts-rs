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
  <img src="https://img.shields.io/github/workflow/status/Aleph-Alpha/ts-rs/Test?style=flat-square" alt="actions status" />
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

## why?
When building a web application in rust, data structures have to be shared between backend and frontend.  
Using this library, you can easily generate TypeScript bindings to your rust structs & enums, so that you can keep your
types in one place.

ts-rs might also come in handy when working with webassembly.

## how?
ts-rs exposes a single trait, `TS`. Using a derive macro, you can implement this interface for your types.  
Then, you can use this trait to obtain the TypeScript bindings.  
We recommend doing this in your tests. [see the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs)

## get started
```toml
[dependencies]
ts-rs = "5.1"
```

```rust
use ts_rs::{TS, export};

#[derive(TS)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
}

export! {
    User => "bindings.ts"
}
```
When running `cargo test`, the TypeScript bindings will be exported to the file `bindings.ts`.

## [example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs)

## features
- generate interface declarations from rust structs
- generate union declarations from rust enums
- inline types
- flatten structs/interfaces
- generate necessary imports when exporting to multiple files
- `export ..` and `declare ..`

## serde compatability
With `serde-compat`, serde attributes can be parsed for enums and structs.  
Supported serde attributes:
- `rename`
- `rename-all`
- `tag`
- `content`
- `untagged`
- `skip`
- `skip_serializing`
- `skip_deserializing`
- `skip_serializing_if = "Option::is_none"`
- `flatten`
- `default`

## todo
- [x] serde compatibility layer
- [x] documentation
- [x] use typescript types across files
- [x] more enum representations
- [x] generics  
- [ ] don't require `'static`
