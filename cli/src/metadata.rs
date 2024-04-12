use std::path::Path;

use color_eyre::{eyre::OptionExt, owo_colors::OwoColorize};

#[derive(Clone)]
pub(super) struct Metadata<'a> {
    pub rust_name: &'a str,
    pub export_path: &'a Path,
}

impl<'a> TryFrom<&'a str> for Metadata<'a> {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (rust_name, export_path) =
            value.split_once(',').ok_or_eyre("Invalid metadata entry")?;

        Ok(Self {
            rust_name,
            export_path: Path::new(export_path),
        })
    }
}

pub(super) fn name_collision_warning(ts_type: &str, metadata: &[Metadata]) {
    eprintln!(
        "{} Multiple types being exported with the name \"{}\"",
        "Warning:".yellow().bold(),
        ts_type.green().bold()
    );

    for entry in metadata {
        eprintln!(
            "  {} {} {}",
            "-".blue().bold(),
            "Type:".bold(),
            entry.rust_name.cyan(),
        );

        eprintln!(
            "    {} {}",
            "Path:".bold(),
            entry.export_path.to_string_lossy()
        );

        eprintln!();
    }

    eprintln!();
}
