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

pub fn remove_empty_subdirectories<T: AsRef<Path>>(path: T) -> Result<()> {
    let path = path.as_ref();

    for entry in path.read_dir()? {
        let entry = entry?;

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        remove_empty_subdirectories(&path)?;
        if let Err(e) = fs::remove_dir(path) {
            // The other possible error kinds are either not available
            // in stable rust (DirectoryNotEmpty), not possible due
            // to the logic of this function (NotFound) or both (NotADirectory)
            //
            // The correct check would be `!matches!(e.kind(), ErrorKind::DirectoryNotEmpty)`
            // as that is the only error we actually WANT to ignore... the others,
            // although impossible, should be returned if they somehow happen
            if matches!(e.kind(), ErrorKind::PermissionDenied) {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
