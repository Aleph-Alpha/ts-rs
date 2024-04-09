use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Defines where your TS bindings will be saved by setting TS_RS_EXPORT_DIR
    #[arg(long, short, default_value = "./bindings")]
    output_directory: PathBuf,

    /// Enables all of ts-rs's feature flags
    #[arg(long)]
    all_features: bool,

    /// Disables serde compatibility
    #[arg(long)]
    disable_serde: bool,

    /// Disables warnings caused by using serde attributes that ts-rs cannot process
    #[arg(long)]
    no_warnings: bool,

    /// Adds the ".js" extension to import paths
    #[arg(long)]
    esm_imports: bool,

    /// Formats the generated TypeScript files
    #[arg(long)]
    format: bool,

    /// Enables chrono compatibility
    #[arg(long)]
    chrono: bool,

    /// Enables bigdecimal compatibility
    #[arg(long)]
    bigdecimal: bool,

    /// Enables uuid compatibility
    #[arg(long)]
    uuid: bool,

    /// Enables bson-uuid compatibility
    #[arg(long)]
    bson_uuid: bool,

    /// Enables bytes compatibility
    #[arg(long)]
    bytes: bool,

    /// Enables url compatibility
    #[arg(long)]
    url: bool,

    /// Enables indexmap compatibility
    #[arg(long)]
    indexmap: bool,

    /// Enables ordered-float compatibility
    #[arg(long)]
    ordered_float: bool,

    /// Enables heapless compatibility
    #[arg(long)]
    heapless: bool,

    /// Enables semver compatibility
    #[arg(long)]
    semver: bool,

    /// Enables serde_json compatibility
    #[arg(long)]
    serde_json: bool,
}

macro_rules! feature {
    ($cargo_invocation: expr, $args: expr, { $($field: ident => $feature: literal),* $(,)? }) => {
        $(
            if $args.$field || $args.all_features {
                $cargo_invocation
                    .arg("--features")
                    .arg(format!("ts-rs/{}", $feature));
            }
        )*
    };
}

fn main() {
    let args = Args::parse();

    if args.disable_serde && args.all_features {
        panic!(r#""--disable-serde" and "--all-features" are not compatible"#)
    }

    let mut cargo_invocation = std::process::Command::new("cargo");

    cargo_invocation
        .arg("test")
        .arg("export_bindings_")
        .env("TS_RS_EXPORT_DIR", args.output_directory);

    if args.disable_serde {
        cargo_invocation.arg("--no-default-features");
    }

    feature!(cargo_invocation, args, {
        no_warnings => "no-serde-warnings",
        esm_imports => "import-esm",
        format => "format",
        chrono => "chrono-impl",
        bigdecimal => "bigdecimal-impl",
        uuid => "uuid-impl",
        bson_uuid => "bson-uuid-impl",
        bytes => "bytes-impl",
        url => "url-impl",
        indexmap => "indexmap-impl",
        ordered_float => "ordered-float-impl",
        heapless => "heapless-impl",
        semver => "semver-impl",
        serde_json => "serde-json-impl"
    });

    cargo_invocation.spawn().unwrap().wait().unwrap();
}
