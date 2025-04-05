use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use clap::Parser;
use color_eyre::{eyre::bail, owo_colors::OwoColorize, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
/// This type wraps `Config` and adds an implementation of
/// the `Drop` trait that deletes the metadata file when
/// the CLI finishes running
pub struct Args(pub Cli);

impl std::ops::Deref for Args {
    type Target = Cli;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Parser, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub enum Cli {
    Init,
    Export(ExportConfig),
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
#[allow(clippy::struct_excessive_bools)]
pub struct ExportConfig {
    #[clap(skip)]
    /// Type overrides for types implemented inside ts-rs.
    pub overrides: HashMap<String, String>,

    /// Path to the `ts-rs` config file
    #[arg(long = "config")]
    #[serde(skip)]
    #[allow(clippy::struct_field_names)]
    pub config_file_path: Option<PathBuf>,

    /// Defines where your TS bindings will be saved by setting `TS_RS_EXPORT_DIR`
    #[arg(long, short)]
    pub output_directory: Option<PathBuf>,

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
    #[serde(rename = "index")]
    pub generate_index_ts: bool,

    /// Do not capture `cargo test`'s output, and pass --nocapture to the test binary
    #[arg(long = "nocapture")]
    #[serde(rename = "nocapture")]
    pub no_capture: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            overrides: HashMap::default(),
            config_file_path: None,
            output_directory: Some(PathBuf::from("./bindings")),
            no_warnings: false,
            esm_imports: false,
            format: false,
            generate_index_ts: false,
            no_capture: false,
        }
    }
}

impl Args {
    pub fn load() -> Result<Self> {
        let cli_args = Cli::parse();

        match cli_args {
            Cli::Init => Ok(Self(cli_args)),
            Cli::Export(export_config) => {
                let config_file_args =
                    ExportConfig::load_from_file(export_config.config_file_path.as_deref())?;

                let cfg = export_config.merge(config_file_args);
                cfg.verify()?;

                Ok(Self(Cli::Export(cfg)))
            }
        }
    }
}

impl ExportConfig {
    pub fn output_directory(&self) -> &Path {
        self.output_directory
            .as_deref()
            .expect("Output directory must not be `None`")
    }

    fn merge(self, other: Self) -> Self {
        Self {
            output_directory: self.output_directory.or(other.output_directory),
            overrides: other.overrides,
            no_warnings: self.no_warnings || other.no_warnings,
            esm_imports: self.esm_imports || other.esm_imports,
            format: self.format || other.format,
            generate_index_ts: self.generate_index_ts || other.generate_index_ts,
            no_capture: self.no_capture || other.no_capture,
            config_file_path: None,
        }
    }

    fn load_from_file(path: Option<&Path>) -> Result<Self> {
        if path.is_some_and(|x| !x.is_file()) {
            bail!("The provided path doesn't exist");
        }

        let path = path.unwrap_or_else(|| Path::new("./ts-rs.toml"));
        if !path.is_file() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    fn verify(&self) -> Result<()> {
        if self.output_directory.is_none() {
            bail!("{}: You must provide the output diretory, either through the config file or the --output-directory flag", "Error".bold().red())
        }

        Ok(())
    }
}
