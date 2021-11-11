# Contributing
If you are unsure what to work on or want to discuss your idea, feel free to open an issue.  

### Documentation
After implementing a new feature, please document it in the doc comment on `TS` in `ts_rs/lib.rs`.  
`README.md` is generated from the module doc comment in `ts_rs/lib.rs`. If you added/updated documentation there, run
`cargo readme > ../README.md` from the `ts_rs/` directory.  
You can install `cargo readme` by running `cargo install cargo-readme`.


### Tests
Please remember to write tests - If you are fixing a bug, write a test first to reproduce it.

### Building
There is nothing special going on here - just run `cargo build`.  
To run the test suite, just run `cargo test` in the root directory.  

### Formatting
To ensure proper formatting, please run `cargo +nightly fmt`.