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
ts-rs = "0.2"
```

```rust
use ts_rs::TS;

#[derive(TS)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
}

#[test]
fn export_ts() {
    std::fs::remove_file("bindings.ts").ok();
    User::dump("bindings.ts").unwrap();
}
```

## [example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs)

## features

- generate interface declarations from rust structs
- generate union declarations from rust enums
- inline types

## serde compatibility layer
With the `serde-compat` feature enabled, ts-rs tries parsing serde attributes.  
Please note that not all serde attributes are supported yet.

## todo

- [x] serde compatibility layer
- [ ] more customization
- [ ] library support (chrono, uuids, ...)
- [x] documentation