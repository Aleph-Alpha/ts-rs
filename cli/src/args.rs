use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Defines where your TS bindings will be saved by setting TS_RS_EXPORT_DIR
    #[arg(long, short)]
    pub output_directory: PathBuf,

    /// Disables warnings caused by using serde attributes that ts-rs cannot process
    #[arg(long)]
    pub no_warnings: bool,

    /// Adds the ".js" extension to import paths
    #[arg(long)]
    pub esm_imports: bool,

    /// Formats the generated TypeScript files
    #[arg(long)]
    pub format: bool,

    /// Generates an index.ts file in your --output-directory that re-exports all
    /// types generated by ts-rs
    #[arg(long = "index")]
    pub generate_index_ts: bool,

    /// Do not capture `cargo test`'s output, and pass --nocapture to the test binary
    #[arg(long = "nocapture")]
    pub no_capture: bool,
}