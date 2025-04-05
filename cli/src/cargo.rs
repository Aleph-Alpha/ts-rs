use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use color_eyre::{eyre::bail, Result};

use crate::{config::ExportConfig, path};

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

pub fn invoke(cfg: &ExportConfig) -> Result<()> {
    let mut cargo_invocation = Command::new("cargo");

    cargo_invocation
        .arg("test")
        .arg("export_bindings_")
        .arg("--features")
        .arg("ts-rs/export")
        .arg("--features")
        .arg("ts-rs/generate-metadata")
        .stdout(if cfg.no_capture {
            Stdio::inherit()
        } else {
            Stdio::piped()
        })
        .env("TS_RS_EXPORT_DIR", path::absolute(cfg.output_directory())?);

    if !cfg.overrides.is_empty() {
        cargo_invocation.env(
            "TS_RS_INTERNAL_OVERRIDE",
            cfg.overrides.iter().fold(String::new(), |acc, (k, v)| {
                format!("{acc}{}{k}:{v}", if acc.is_empty() { "" } else { ";" })
            }),
        );
    }

    feature!(cargo_invocation, cfg, {
        no_warnings => "no-serde-warnings",
        esm_imports => "import-esm",
        format => "format",
    });

    if cfg.no_capture {
        cargo_invocation.arg("--").arg("--nocapture");
    } else {
        cargo_invocation.arg("--quiet");
    }

    cargo_invocation.spawn()?.wait()?;

    Ok(())
}

pub fn workspace_location() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()?;

    match output.status.code() {
        Some(0) => Ok(PathBuf::from(std::str::from_utf8(&output.stdout)?)
            .parent()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| PathBuf::from("/"))),
        Some(_) => bail!("{}", std::str::from_utf8(&output.stderr)?),
        None => bail!("Unable to obtain workspace path"),
    }
}
