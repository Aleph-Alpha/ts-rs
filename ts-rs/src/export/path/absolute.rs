use std::{
    env,
    path::{Path, PathBuf},
};

use super::clean::PathClean;

/// This trait is responsible for converting a relative path
/// into an absolute path
pub trait PathAbsolute: AsRef<Path> + PathClean {
    /// Converts a relative path into an absolute path.
    fn absolute(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(self).clean()
    }
}

impl<T: AsRef<Path> + PathClean> PathAbsolute for T {}
