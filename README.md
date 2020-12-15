# ts-rs

Generate TypeScript interface/type declarations from rust structs.  

[![Crate](https://img.shields.io/crates/v/ts-rs.svg)](https://crates.io/crates/ts-rs)
[![API](https://docs.rs/ts-rs/badge.svg)](https://docs.rs/ts-rs)

## why?

When building a web application in rust, data structures have to be shared between backend and frontend.  
Using this library, you can easily generate TypeScript bindings to your rust structs & enums, so that you can keep your
types in one place.

ts-rs might also come in handy when working with webassembly.

## how?

ts-rs exposes a single interface, `TS`. Using a derive macro, you can implement this interface for your types.  
Then, you can use this trait to obtain the TypeScript bindings.  
We recommend doing this in your tests. [see the example](https://google.de)

## [example](https://google.de)

## features

- generate interface declarations from rust structs
- generate union declarations from rust enums
- inline types

## todo

-[ ] serde compatibility layer
-[ ] more customization
-[ ] library support (chrono, uuids, ...)
