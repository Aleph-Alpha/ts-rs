use std::path::{Component as C, Path, PathBuf};

use super::ExportError as E;

const ERROR_MESSAGE: &str = r#"The path provided with `#[ts(export_to = "..")]` is not valid"#;
pub trait PathAbsolute: AsRef<Path> {
    fn absolute(&self) -> Result<PathBuf, E> {
        let original_path = self.as_ref();

        if original_path.is_absolute() {
            return Ok(original_path.to_owned());
        }

        let path = std::env::current_dir()?.join(original_path);

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
}

impl<T: AsRef<Path>> PathAbsolute for T {}
