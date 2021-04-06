<img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/ts-rs/main/logo.png">  

# ts-rs  
Generate TypeScript interface/type declarations from rust structs.  

[![Crate](https://img.shields.io/crates/v/ts-rs.svg)](https://crates.io/crates/ts-rs)
[![API](https://docs.rs/ts-rs/badge.svg)](https://docs.rs/ts-rs)
[![Test](https://github.com/Aleph-Alpha/ts-rs/workflows/Test/badge.svg)](https://github.com/Aleph-Alpha/ts-rs/actions)

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
ts-rs = "2.3"
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

## Serde Compatability
Serde representations are supported for enums and structs. The serde tags supported on enums are:
- tag & content
- untagged


## todo

- [x] serde compatibility layer
- [x] documentation
- [x] use typescript types across files
- [x] more enum representations
- [ ] don't require `'static`
