use std::{
    env,
    io::Result,
    path::{Path, PathBuf},
};

use super::clean::PathClean;

/// This trait is responsible for converting a relative path
/// into an absolute path
pub trait PathAbsolute: AsRef<Path> + PathClean {
    /// Converts a relative path into an absolute path.
    fn absolute(&self) -> Result<PathBuf> {
        Ok(Path::new(&env::current_dir()?).join(self).clean())
    }
}

impl<T: AsRef<Path> + PathClean> PathAbsolute for T {}
