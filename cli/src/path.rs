use std::path::{Component as C, Path, PathBuf};

use color_eyre::{eyre::OptionExt, Result};

pub fn absolute<T: AsRef<Path>>(path: T) -> Result<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        return Ok(path.to_owned());
    }

    let path = std::env::current_dir()?.join(path);

    let mut out = Vec::new();
    for comp in path.components() {
        match comp {
            C::CurDir => (),
            C::ParentDir => {
                out.pop().ok_or_eyre("Invalid path")?;
            }
            comp => out.push(comp),
        }
    }

    Ok(if out.is_empty() {
        PathBuf::from(".")
    } else {
        out.iter().collect()
    })
}
