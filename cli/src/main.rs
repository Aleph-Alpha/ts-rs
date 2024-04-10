use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Defines where your TS bindings will be saved by setting TS_RS_EXPORT_DIR
    #[arg(long, short, default_value = "./bindings")]
    output_directory: PathBuf,

    /// Disables warnings caused by using serde attributes that ts-rs cannot process
    #[arg(long)]
    no_warnings: bool,

    /// Adds the ".js" extension to import paths
    #[arg(long)]
    esm_imports: bool,

    /// Formats the generated TypeScript files
    #[arg(long)]
    format: bool,
}

macro_rules! feature {
    ($cargo_invocation: expr, $args: expr, { $($field: ident => $feature: literal),* $(,)? }) => {
        $(
            if $args.$field {
                $cargo_invocation
                    .arg("--features")
                    .arg(format!("ts-rs/{}", $feature));
            }
        )*
    };
}

fn main() {
    let args = Args::parse();

    let mut cargo_invocation = std::process::Command::new("cargo");

    cargo_invocation
        .arg("test")
        .arg("export_bindings_")
        .arg("--features")
        .arg("ts-rs/export")
        .env("TS_RS_EXPORT_DIR", args.output_directory);

    feature!(cargo_invocation, args, {
        no_warnings => "no-serde-warnings",
        esm_imports => "import-esm",
        format => "format",
    });

    cargo_invocation.spawn().unwrap().wait().unwrap();
}
