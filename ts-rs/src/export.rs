use std::{
    any::TypeId,
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Write,
    fs::File,
    io::{Seek, SeekFrom},
    path::{Component, Path, PathBuf},
    sync::Mutex,
};

pub use error::Error;
use lazy_static::lazy_static;
use path::diff_paths;
pub(crate) use recursive_export::export_all_into;

use crate::TS;

mod error;
mod path;

lazy_static! {
    static ref EXPORT_PATHS: Mutex<HashMap<PathBuf, HashSet<String>>> = Mutex::new(HashMap::new());
}

const NOTE: &str = "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n";

mod recursive_export {
    use std::{any::TypeId, collections::HashSet, path::Path};

    use super::export_into;
    use crate::{Error, TypeVisitor, TS};

    /// Exports `T` to the file specified by the `#[ts(export_to = ..)]` attribute within the given
    /// base directory.  
    /// Additionally, all dependencies of `T` will be exported as well.
    pub(crate) fn export_all_into<T: TS + ?Sized + 'static>(
        out_dir: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let mut seen = HashSet::new();
        export_recursive::<T>(&mut seen, out_dir)
    }

    struct Visit<'a> {
        seen: &'a mut HashSet<TypeId>,
        out_dir: &'a Path,
        error: Option<Error>,
    }

    impl<'a> TypeVisitor for Visit<'a> {
        fn visit<T: TS + 'static + ?Sized>(&mut self) {
            // if an error occurred previously, or the type cannot be exported (it's a primitive),
            // we return
            if self.error.is_some() || T::output_path().is_none() {
                return;
            }

            self.error = export_recursive::<T>(self.seen, self.out_dir).err();
        }
    }

    // exports T, then recursively calls itself with all of its dependencies
    fn export_recursive<T: TS + ?Sized + 'static>(
        seen: &mut HashSet<TypeId>,
        out_dir: impl AsRef<Path>,
    ) -> Result<(), Error> {
        if !seen.insert(TypeId::of::<T>()) {
            return Ok(());
        }
        let out_dir = out_dir.as_ref();

        export_into::<T>(out_dir)?;

        let mut visitor = Visit {
            seen,
            out_dir,
            error: None,
        };
        T::visit_dependencies(&mut visitor);

        if let Some(e) = visitor.error {
            Err(e)
        } else {
            Ok(())
        }
    }
}

/// Export `T` to the file specified by the `#[ts(export_to = ..)]` attribute
pub(crate) fn export_into<T: TS + ?Sized + 'static>(
    out_dir: impl AsRef<Path>,
) -> Result<(), Error> {
    let path = T::output_path()
        .ok_or_else(std::any::type_name::<T>)
        .map_err(Error::CannotBeExported)?;
    let path = out_dir.as_ref().join(path);

    export_to::<T, _>(path::absolute(path)?)
}

/// Export `T` to the file specified by the `path` argument.
pub(crate) fn export_to<T: TS + ?Sized + 'static, P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let path = path.as_ref().to_owned();
    let type_name = T::ident();

    #[allow(unused_mut)]
    let mut buffer = export_to_string::<T>()?;

    // format output
    #[cfg(feature = "format")]
    {
        use dprint_plugin_typescript::{configuration::ConfigurationBuilder, format_text};

        let fmt_cfg = ConfigurationBuilder::new().deno().build();
        if let Some(formatted) = format_text(path.as_ref(), &buffer, &fmt_cfg)
            .map_err(|e| Error::Formatting(e.to_string()))?
        {
            buffer = formatted;
        }
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    {
        use std::io::{Read, Write};

        let mut lock = EXPORT_PATHS.lock().unwrap();

        if let Some(entry) = lock.get_mut(&path) {
            if entry.contains(&type_name) {
                return Ok(());
            }

            let (header, decl) = buffer.split_once("\n\n").unwrap();
            let imports = if header.len() > NOTE.len() {
                &header[NOTE.len()..]
            } else {
                ""
            };

            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)?;

            file.seek(SeekFrom::Start(NOTE.len().try_into().unwrap()))?;

            let content_len = usize::try_from(file.metadata()?.len()).unwrap() - NOTE.len();
            let mut original_contents = String::with_capacity(content_len);
            file.read_to_string(&mut original_contents)?;

            let imports = imports
                .lines()
                .filter(|x| !original_contents.contains(x))
                .collect::<String>();

            file.seek(SeekFrom::Start(NOTE.len().try_into().unwrap()))?;

            let buffer_size = imports.as_bytes().len() + decl.as_bytes().len() + content_len + 3;

            let mut buffer = String::with_capacity(buffer_size);

            buffer.push_str(&imports);
            if !imports.is_empty() {
                buffer.push('\n');
            }
            buffer.push_str(&original_contents);
            buffer.push_str("\n\n");
            buffer.push_str(decl);

            file.write_all(buffer.as_bytes())?;

            entry.insert(type_name);
        } else {
            let mut file = File::create(&path)?;
            file.write_all(buffer.as_bytes())?;
            file.sync_data()?;

            let mut set = HashSet::new();
            set.insert(type_name);
            lock.insert(path, set);
        }
    }

    Ok(())
}

/// Returns the generated definition for `T`.
pub(crate) fn export_to_string<T: TS + ?Sized + 'static>() -> Result<String, Error> {
    let mut buffer = String::with_capacity(1024);
    buffer.push_str(NOTE);
    generate_imports::<T::WithoutGenerics>(&mut buffer, default_out_dir())?;
    generate_decl::<T>(&mut buffer);
    Ok(buffer)
}

pub(crate) fn default_out_dir() -> Cow<'static, Path> {
    match std::env::var("TS_RS_EXPORT_DIR") {
        Err(..) => Cow::Borrowed(Path::new("./bindings")),
        Ok(dir) => Cow::Owned(PathBuf::from(dir)),
    }
}

/// Push the declaration of `T`
fn generate_decl<T: TS + ?Sized>(out: &mut String) {
    // Type Docs
    let docs = &T::DOCS;
    if let Some(docs) = docs {
        out.push_str(docs);
    }

    // Type Definition
    out.push_str("export ");
    out.push_str(&T::decl());
}

/// Push an import statement for all dependencies of `T`.
fn generate_imports<T: TS + ?Sized + 'static>(
    out: &mut String,
    out_dir: impl AsRef<Path>,
) -> Result<(), Error> {
    let path = T::output_path()
        .ok_or_else(std::any::type_name::<T>)
        .map(|x| out_dir.as_ref().join(x))
        .map_err(Error::CannotBeExported)?;

    let deps = T::dependencies();
    let deduplicated_deps = deps
        .iter()
        .filter(|dep| dep.type_id != TypeId::of::<T>())
        .map(|dep| (&dep.ts_name, dep))
        .collect::<BTreeMap<_, _>>();

    for (_, dep) in deduplicated_deps {
        let dep_path = out_dir.as_ref().join(dep.output_path);
        let rel_path = import_path(&path, &dep_path)?;

        let is_same_file = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(|x| x.trim_end_matches(".ts"))
            .map(|x| format!("./{x}"))
            .map(|x| x == rel_path.trim_end_matches(".js"))
            .unwrap_or(false);

        if is_same_file {
            continue;
        }

        writeln!(
            out,
            "import type {{ {} }} from {:?};",
            &dep.ts_name, rel_path
        )?;
    }
    writeln!(out)?;
    Ok(())
}

/// Returns the required import path for importing `import` from the file `from`
fn import_path(from: &Path, import: &Path) -> Result<String, Error> {
    let rel_path = diff_paths(import, from.parent().unwrap())?;
    let path = match rel_path.components().next() {
        Some(Component::Normal(_)) => format!("./{}", rel_path.to_string_lossy()),
        _ => rel_path.to_string_lossy().into(),
    };

    let path_without_extension = path.trim_end_matches(".ts");

    Ok(if cfg!(feature = "import-esm") {
        format!("{}.js", path_without_extension)
    } else {
        path_without_extension.to_owned()
    })
}
