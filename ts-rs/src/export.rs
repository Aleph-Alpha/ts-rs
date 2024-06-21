use std::{
    any::TypeId,
    borrow::Cow,
    collections::BTreeMap,
    fmt::Write,
    fs::File,
    path::{Component, Path, PathBuf},
    sync::Mutex,
};

pub(crate) use recursive_export::export_all_into;
use thiserror::Error;

use crate::TS;

const NOTE: &str = "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n";

/// An error which may occur when exporting a type
#[derive(Error, Debug)]
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
}

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

    export_to::<T, _>(std::path::absolute(path)?)
}

/// Export `T` to the file specified by the `path` argument.
pub(crate) fn export_to<T: TS + ?Sized + 'static, P: AsRef<Path>>(
    path: P,
) -> Result<(), ExportError> {
    // Lock to make sure only one file will be written at a time.
    // In the future, it might make sense to replace this with something more clever to only prevent
    // two threads from writing the **same** file concurrently.
    static FILE_LOCK: Mutex<()> = Mutex::new(());

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

    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }
    let lock = FILE_LOCK.lock().unwrap();
    {
        // Manually write to file & call `sync_data`. Otherwise, calling `fs::read(path)`
        // immediately after `T::export()` might result in an empty file.
        use std::io::Write;
        let mut file = File::create(path)?;
        file.write_all(buffer.as_bytes())?;
        file.sync_data()?;
    }

    drop(lock);
    Ok(())
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
        .map_err(ExportError::CannotBeExported)?;
    let path = out_dir.as_ref().join(path);

    let deps = T::dependencies();
    let deduplicated_deps = deps
        .iter()
        .filter(|dep| dep.type_id != TypeId::of::<T>())
        .map(|dep| (&dep.ts_name, dep))
        .collect::<BTreeMap<_, _>>();

    for (_, dep) in deduplicated_deps {
        let dep_path = out_dir.as_ref().join(dep.output_path);
        let rel_path = import_path(&path, &dep_path);
        writeln!(
            out,
            "import type {{ {} }} from {:?};",
            &dep.ts_name, rel_path
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    Ok(())
}

/// Returns the required import path for importing `import` from the file `from`
fn import_path(from: &Path, import: &Path) -> String {
    let rel_path =
        diff_paths(import, from.parent().unwrap()).expect("failed to calculate import path");
    let path = match rel_path.components().next() {
        Some(Component::Normal(_)) => format!("./{}", rel_path.to_string_lossy()),
        _ => rel_path.to_string_lossy().into(),
    };

    let path_without_extension = path.trim_end_matches(".ts");

    if cfg!(feature = "import-esm") {
        format!("{}.js", path_without_extension)
    } else {
        path_without_extension.to_owned()
    }
}

// Construct a relative path from a provided base directory path to the provided path.
//
// Copyright 2012-2015 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Adapted from rustc's path_relative_from
// https://github.com/rust-lang/rust/blob/e1d0de82cc40b666b88d4a6d2c9dcbc81d7ed27f/src/librustc_back/rpath.rs#L116-L158
fn diff_paths<P, B>(path: P, base: B) -> Result<PathBuf, ExportError>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    use Component as C;

    let path = std::path::absolute(path)?;
    let base = std::path::absolute(base)?;

    let mut ita = path.components();
    let mut itb = base.components();
    let mut comps: Vec<Component> = vec![];

    loop {
        match (ita.next(), itb.next()) {
            (Some(C::ParentDir | C::CurDir), _) | (_, Some(C::ParentDir | C::CurDir)) => {
                unreachable!(
                    "The paths have been cleaned, no no '.' or '..' components should present\n path: {path:?}\nbase: {base:?}"
                )
            }
            (None, None) => break,
            (Some(a), None) => {
                comps.push(a);
                comps.extend(ita.by_ref());
                break;
            }
            (None, _) => comps.push(Component::ParentDir),
            (Some(a), Some(b)) if comps.is_empty() && a == b => (),
            (Some(a), Some(_)) => {
                comps.push(Component::ParentDir);
                for _ in itb {
                    comps.push(Component::ParentDir);
                }
                comps.push(a);
                comps.extend(ita.by_ref());
                break;
            }
        }
    }

    Ok(comps.iter().map(|c| c.as_os_str()).collect())
}
