#![warn(clippy::pedantic, clippy::nursery)]

use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use cargo::workspace_location;
use color_eyre::{owo_colors::OwoColorize, Result};

mod cargo;
mod config;
mod metadata;
mod path;

use config::{Cli, ExportConfig};
use metadata::{Metadata, FILE_NAME};

use crate::config::Args;

const NOTE: &[u8; 109] = b"// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n";

impl Drop for Args {
    fn drop(&mut self) {
        if let Self(Cli::Export(config)) = self {
            _ = fs::remove_file(config.output_directory().join(FILE_NAME));
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::load()?;

    match args.0 {
        Cli::Init => {
            let path = workspace_location()?.join("ts-rs.toml");
            if path.exists() {
                eprintln!("{} the config file already exists", "Error:".red().bold());
                return Ok(());
            }

            let mut file = OpenOptions::new().create(true).append(true).open(path)?;

            let config = ExportConfig::default();

            write!(file, "{}", toml::to_string_pretty(&config)?)?;

            Ok(())
        }
        Cli::Export(ref args) => {
            let metadata_path = args.output_directory().join(FILE_NAME);
            if metadata_path.exists() {
                fs::remove_file(&metadata_path)?;
            }

            cargo::invoke(args)?;

            let metadata_content = fs::read_to_string(&metadata_path)?;
            let metadata = Metadata::try_from(&*metadata_content)?;

            if !args.generate_index_ts || metadata.is_empty() {
                return Ok(());
            }

            if metadata.has_naming_collisions() {
                metadata.report_naming_collisions();

                eprintln!(
                    "{} due to the naming collisions listed above, generating an index.ts file is not possible",
                    "Error:".red().bold()
                );

                return Ok(());
            }

            let index_path = args.output_directory().join("index.ts");

            if index_path.exists() {
                fs::remove_file(&index_path)?;
            }

            let mut index = OpenOptions::new()
                .create(true)
                .append(true)
                .open(index_path)?;

            index.write_all(NOTE)?;

            if args.generate_index_ts {
                for path in metadata.export_paths() {
                    index.write_fmt(format_args!("\nexport * from {path:?};"))?;
                }

                return Ok(());
            }

            Ok(())
        }
    }
}
