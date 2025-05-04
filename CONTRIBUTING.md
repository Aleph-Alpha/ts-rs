# Contributing
If you are unsure what to work on or want to discuss your idea, feel free to open an issue.  

### Documentation
After implementing a new feature, please document it in the doc comment on `TS` in `ts_rs/lib.rs`.  
`README.md` is generated from the module doc comment in `ts_rs/lib.rs`. If you added/updated documentation there, go to the `ts-rs/` directory and run one of the following commands:

On Windows:
`cargo readme -o ..\README.md`

On other systems:
`cargo readme > ../README.md`

You can install `cargo readme` by running `cargo install cargo-readme`.


### Tests
Please remember to write tests - If you are fixing a bug, write a test first to reproduce it.

### Building
There is nothing special going on here - just run `cargo build`.  
To run the test suite, just run `cargo test` in the root directory.  

### Formatting
To ensure proper formatting, please make sure you have the nigthly toolchain installed.
After that, in the project's root directory, create a file called `.git/hooks/pre-commit` without a file extension and add the following two lines:
```sh
#!/bin/sh
cargo +nightly fmt
```

This will make sure your files are formatted before your commit is sent, so you don't have to manually run `cargo +nightly fmt`
