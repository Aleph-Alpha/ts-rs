/// An error which may occur when exporting a type
#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    #[error("this type cannot be exported")]
    CannotBeExported(&'static str),
    #[cfg(feature = "format")]
    #[error("an error occurred while formatting the generated typescript output")]
    Formatting(String),
    #[error("an error occurred while performing IO")]
    Io(#[from] std::io::Error),
    #[error("the environment variable CARGO_MANIFEST_DIR is not set")]
    ManifestDirNotSet,
    #[error("an error occurred while writing to a formatted buffer")]
    Fmt(#[from] std::fmt::Error),
    #[error(r#"TS_RS_IMPORT_EXTENSION must be either "js" or "ts""#)]
    InvalidImportExtension,
    #[error(
        "type name collision for \"{new_type}\" in {}: two different types generate different \
         content for the same file. Use #[ts(rename = \"...\")] or #[ts(export_to = \"...\")] \
         to disambiguate, or set TS_RS_AUTO_NAMESPACE=true to organize types by crate.",
        path.display()
    )]
    Collision {
        path: std::path::PathBuf,
        existing_types: String,
        new_type: String,
    },
}
