use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use color_eyre::{
    eyre::{Error, OptionExt},
    owo_colors::OwoColorize,
    Result,
};

pub const FILE_NAME: &str = "ts_rs.meta";

pub struct Metadata<'a> {
    entries: HashMap<&'a str, HashSet<Entry<'a>>>,
}

impl<'a> TryFrom<&'a str> for Metadata<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            entries: value.lines().try_fold(
                HashMap::<&str, HashSet<_>>::default(),
                |mut acc, cur| {
                    let (key, value) = cur.split_once(',').ok_or_eyre("Invalid metadata file")?;
                    let value = Entry::try_from(value)?;

                    acc.entry(key).or_default().insert(value);

                    Ok::<_, Error>(acc)
                },
            )?,
        })
    }
}

impl Metadata<'_> {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn has_naming_collisions(&self) -> bool {
        self.entries.values().any(|x| x.len() > 1)
    }

    pub fn report_naming_collisions(&self) {
        self.entries
            .iter()
            .filter(|(_, x)| x.len() > 1)
            .for_each(|(ty, entry)| name_collision_warning(ty, entry));
    }

    pub fn export_paths(&self) -> impl Iterator<Item = &Path> {
        self.entries.values().flatten().map(|x| x.export_path)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Entry<'a> {
    rust_name: &'a str,
    export_path: &'a Path,
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (rust_name, export_path) =
            value.split_once(',').ok_or_eyre("Invalid metadata entry")?;

        Ok(Self {
            rust_name,
            export_path: Path::new(export_path),
        })
    }
}

fn name_collision_warning(ts_type: &str, metadata: &HashSet<Entry>) {
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
