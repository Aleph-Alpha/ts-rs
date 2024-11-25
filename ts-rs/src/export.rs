use std::{
    any::TypeId,
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Write,
    fs::File,
    io::{Seek, SeekFrom},
    path::{Component, Path, PathBuf},
    sync::Mutex,
};

pub use error::ExportError;
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
    use crate::{ExportError, TypeVisitor, TS};

    /// Exports `T` to the file specified by the `#[ts(export_to = ..)]` attribute within the given
    /// base directory.  
    /// Additionally, all dependencies of `T` will be exported as well.
    pub(crate) fn export_all_into<T: TS + ?Sized + 'static>(
        out_dir: impl AsRef<Path>,
    ) -> Result<(), ExportError> {
        let mut seen = HashSet::new();
        export_recursive::<T>(&mut seen, out_dir)
    }

    struct Visit<'a> {
        seen: &'a mut HashSet<TypeId>,
        out_dir: &'a Path,
        error: Option<ExportError>,
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
    ) -> Result<(), ExportError> {
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
) -> Result<(), ExportError> {
    let path = T::output_path()
        .ok_or_else(std::any::type_name::<T>)
        .map_err(ExportError::CannotBeExported)?;
    let path = out_dir.as_ref().join(path);

    export_to::<T, _>(path::absolute(path)?)
}

/// Export `T` to the file specified by the `path` argument.
pub(crate) fn export_to<T: TS + ?Sized + 'static, P: AsRef<Path>>(
    path: P,
) -> Result<(), ExportError> {
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
            .map_err(|e| ExportError::Formatting(e.to_string()))?
        {
            buffer = formatted;
        }
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    export_and_merge(path, type_name, buffer)?;

    Ok(())
}

/// Exports the type to a new file if the file hasn't yet been written to.
/// Otherwise, finds its place in the already existing file and inserts it.
fn export_and_merge(
    path: PathBuf,
    type_name: String,
    generated_type: String,
) -> Result<(), ExportError> {
    use std::io::{Read, Write};

    let mut lock = EXPORT_PATHS.lock().unwrap();

    let Some(entry) = lock.get_mut(&path) else {
        // The file hasn't been written to yet, so it must be
        // overwritten
        let mut file = File::create(&path)?;
        file.write_all(generated_type.as_bytes())?;
        file.sync_all()?;

        let mut set = HashSet::new();
        set.insert(type_name);
        lock.insert(path, set);

        return Ok(());
    };

    if entry.contains(&type_name) {
        return Ok(());
    }

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)?;

    let file_len = file.metadata()?.len();

    let mut original_contents = String::with_capacity(file_len as usize);
    file.read_to_string(&mut original_contents)?;

    let buffer = merge(original_contents, generated_type);

    file.seek(SeekFrom::Start(NOTE.len() as u64))?;

    file.write_all(buffer.as_bytes())?;
    file.sync_all()?;

    entry.insert(type_name);

    Ok(())
}

const HEADER_ERROR_MESSAGE: &str = "The generated strings must have their NOTE and imports separated from their type declarations by a new line";

const DECLARATION_START: &str = "export type ";

/// Inserts the imports and declaration from the newly generated type
/// into the contents of the file, removimg duplicate imports and organazing
/// both imports and declarations alphabetically
fn merge(original_contents: String, new_contents: String) -> String {
    let (original_header, original_decls) = original_contents
        .split_once("\n\n")
        .expect(HEADER_ERROR_MESSAGE);
    let (new_header, new_decl) = new_contents.split_once("\n\n").expect(HEADER_ERROR_MESSAGE);

    let imports = original_header
        .lines()
        .skip(1)
        .chain(new_header.lines().skip(1))
        .collect::<BTreeSet<_>>();

    let import_len = imports.iter().map(|&x| x.len()).sum::<usize>() + imports.len();
    let capacity = import_len + original_decls.len() + new_decl.len() + 2;

    let mut buffer = String::with_capacity(capacity);

    for import in imports {
        buffer.push_str(import);
        buffer.push('\n')
    }

    let new_decl = new_decl.trim_matches('\n');

    let new_decl_name = new_decl
        .split(DECLARATION_START)
        .last()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();

    let original_decls = original_decls.split("\n\n").map(|x| x.trim_matches('\n'));

    let mut inserted = false;
    for decl in original_decls {
        let decl_name = decl
            .split(DECLARATION_START)
            .last()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();

        if inserted || decl_name < new_decl_name {
            buffer.push('\n');
            buffer.push_str(decl);
            buffer.push('\n');
        } else {
            buffer.push('\n');
            buffer.push_str(new_decl);
            buffer.push('\n');

            buffer.push('\n');
            buffer.push_str(decl);
            buffer.push('\n');

            inserted = true;
        }
    }

    if !inserted {
        buffer.push('\n');
        buffer.push_str(new_decl);
        buffer.push('\n');
    }

    buffer
}

/// Returns the generated definition for `T`.
pub(crate) fn export_to_string<T: TS + ?Sized + 'static>() -> Result<String, ExportError> {
    let mut buffer = String::with_capacity(1024);
    buffer.push_str(NOTE);
    generate_imports::<T::WithoutGenerics>(&mut buffer, default_out_dir())?;
    generate_decl::<T>(&mut buffer);
    buffer.push('\n');
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
) -> Result<(), ExportError> {
    let path = T::output_path()
        .ok_or_else(std::any::type_name::<T>)
        .map(|x| out_dir.as_ref().join(x))
        .map_err(ExportError::CannotBeExported)?;

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
            r#"import type {{ {} }} from "{}";"#,
            &dep.ts_name, rel_path
        )?;
    }
    writeln!(out)?;
    Ok(())
}

/// Returns the required import path for importing `import` from the file `from`
fn import_path(from: &Path, import: &Path) -> Result<String, ExportError> {
    let rel_path = diff_paths(import, from.parent().unwrap())?;
    let str_path = match rel_path.components().next() {
        Some(Component::Normal(_)) => {
            format!("./{}", rel_path.to_string_lossy())
        }
        _ => rel_path.to_string_lossy().into(),
    };

    let path = if cfg!(target_os = "windows") {
        str_path.replace('\\', "/")
    } else {
        str_path
    };

    let path_without_extension = path.trim_end_matches(".ts");

    Ok(if cfg!(feature = "import-esm") {
        format!("{}.js", path_without_extension)
    } else {
        path_without_extension.to_owned()
    })
}
