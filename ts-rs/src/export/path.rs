use std::path::{Component as C, Path, PathBuf};

use super::ExportError as E;

const ERROR_MESSAGE: &str = r#"The path provided with `#[ts(export_to = "..")]` is not valid"#;
pub fn absolute<T: AsRef<Path>>(path: T) -> Result<PathBuf, E> {
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
                out.pop().ok_or(E::CannotBeExported(ERROR_MESSAGE))?;
            }
            comp => out.push(comp),
        }
    }

    Ok(if !out.is_empty() {
        out.iter().collect()
    } else {
        PathBuf::from(".")
    })
}
