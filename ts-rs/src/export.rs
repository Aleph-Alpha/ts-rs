use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use crate::TS;
use std::fmt::Write;

/// Placeholder type that can be used to instruct `export!` to generate an index file that
/// re-exports all generated types.
///
/// ```rust
/// use ts_rs::{export, index};
///
/// export ! {
///     A => "bindings/a.ts",
///     index => "bindings/index.ts" // export type { A } from "./a.ts";
/// }
/// ```
#[allow(non_camel_case_types)]
pub type index = ();

/// Expands to a test function which exports typescript bindings to one or multiple files when
/// running `cargo test`.  
/// If a type depends on an other type which is exported to a different file, an appropriate import
/// will be included.  
/// If a file already exists, it will be overriden.  
/// Missing parent directories of the file(s) will be created.  
/// Paths are interpreted as being relative to the project root.
/// ```rust
/// # use ts_rs::{export, TS};
/// #[derive(TS)] struct A;
/// #[derive(TS)] struct B;
/// #[derive(TS)] struct C;
///
/// export! {
///     A, B => "bindings/a.ts",
///     C => "bindings/b.ts"
/// }
/// ```
/// When running `cargo test`, bindings for `A`, `B` and `C` will be exported to `bindings/a.ts`
/// and `bindings/b.ts`.
///
/// ## Ambient declarations
///
/// By default, `export!` always uses `export type/interface`.
/// If you wish, you can also use ambient declarations (`declare type/interface`):
/// ```rust
/// # use ts_rs::{export, TS};
/// #[derive(TS)] struct Declared;
/// #[derive(TS)] struct Normal(Declared);
///
/// export! {
///     (declare) Declared => "bindings/declared.d.ts",
///     Normal => "bindings/normal.ts"
/// }
/// ```
/// Since `Declared` is now an ambient declaration, `bindings/normal.ts` will not include an import
/// for `bindings/declared.d.ts`.
///
/// ## Index file
///
/// You may also wish to generate an index file that re-exports all generated types. You can do so
/// using the special `index` export:
///
/// ```rust
/// use ts_rs::{export, index};
///
/// export ! {
///     A => "bindings/a.ts",
///     index => "bindings/index.ts" // export type { A } from "./a.ts";
/// }
/// ```
#[macro_export]
macro_rules! export {
    ($($(($decl:ident))? $($p:path),+ => $l:literal),* $(,)?) => {
        #[cfg(test)]
        #[test]
        fn export_typescript() {
            use std::fmt::Write;
            use std::collections::{BTreeMap as __BTreeMap, BTreeSet as __BTreeSet};

            let index_type = std::any::TypeId::of::<ts_rs::export::index>();

            let manifest_var = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let manifest_dir = std::path::Path::new(&manifest_var);

            // {TypeId} -> {PathBuf}
            let mut files = __BTreeMap::new();
            $(
                let path = manifest_dir.join($l);

                // if the type(s) should be `declare`d, then they should not be imported.
                let mut declared = false;
                $( ts_rs::check_declare!($decl); declared = true; )*;

                if !declared {
                    $({
                        if let Some(_) = files.insert(std::any::TypeId::of::<$p>(), path.clone()) {
                            panic!(
                                "{} cannot be exported to multiple files using `export!`",
                                stringify!($p),
                            );
                        }
                    })*
                }
            )*

            let mut buffer = String::with_capacity(8192);
            let mut imports = __BTreeMap::<String, __BTreeSet<String>>::new();
            let fmt_config = ts_rs::export::FmtCfg::new() .deno().build();

            $({
                // clear buffers
                buffer.clear();
                imports.clear();

                // create output directory
                let out = manifest_dir.join($l);
                std::fs::create_dir_all(out.parent().unwrap())
                    .expect("could not create directory");

                // write imports
                $(
                    if std::any::TypeId::of::<$p>() != index_type {
                        ts_rs::export::imports::<$p>(&files, &mut imports, &out);
                    }
                )*
                ts_rs::export::write_imports(&imports, &mut buffer);
                buffer.push_str("\n");

                // write declarations

                // check if `export` or `declare` should be used
                let mut prefix = "export ";
                $( ts_rs::check_declare!($decl); prefix = "declare "; )*;

                $(
                    if std::any::TypeId::of::<$p>() != index_type {
                        buffer.push_str(prefix);
                        buffer.push_str(&<$p as ts_rs::TS>::decl());
                        buffer.push_str("\n\n");
                    }
                )*

                if !buffer.trim().is_empty() {
                    // format output
                    let buffer = ts_rs::export::fmt_ts(&out, &buffer, &fmt_config)
                        .expect("could not format output");

                    std::fs::write(&out, buffer.trim())
                        .expect("could not write file");
                }
            })*

            // generate index, if requested
            if let Some(index_path) = files.get(&index_type) {
                buffer.clear();

                let out = manifest_dir.join(index_path);
                std::fs::create_dir_all(out.parent().unwrap()).expect("could not create directory");

                // generate the content of the index file
                $({
                    let mut types = vec![];
                    $(
                        if std::any::TypeId::of::<$p>() != index_type {
                            types.push(<$p as ts_rs::TS>::name());
                        }
                    )*

                    if !types.is_empty() {
                        let path = manifest_dir.join($l);
                        writeln!(
                            &mut buffer,
                            "export type {{{}}} from {:?};",
                            types
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", "),
                            ts_rs::export::import_path(index_path, &path)
                        )
                        .unwrap();
                    }
                })*

                let buffer = ts_rs::export::fmt_ts(&out, &buffer, &fmt_config).expect("could not format output");

                std::fs::write(&out, buffer.trim()).expect("could not write file");
            }
        }
    };
}

// checks that the given argument is `declare`, emitting a compile_error! if it isn't.
#[doc(hidden)]
#[macro_export]
macro_rules! check_declare {
    (declare) => {};
    ($x:ident) => {
        compile_error!(concat!(
            "expected `(declare)`, got `(",
            stringify!($x),
            ")`"
        ));
    };
}

pub use dprint_plugin_typescript::{
    configuration::ConfigurationBuilder as FmtCfg, format_text as fmt_ts,
};

pub fn write_imports(imports: &BTreeMap<String, BTreeSet<String>>, out: &mut impl Write) {
    for (path, types) in imports {
        writeln!(
            out,
            "import {{{}}} from {:?};",
            types.iter().cloned().collect::<Vec<_>>().join(", "),
            path
        )
        .unwrap();
    }
}

pub fn imports<T: TS>(
    exported_files: &BTreeMap<TypeId, PathBuf>,
    imports: &mut BTreeMap<String, BTreeSet<String>>,
    out_path: &Path,
) {
    T::dependencies()
        .into_iter()
        .flat_map(|(id, name)| {
            let path = exported_files.get(&id)?;
            if path == out_path {
                None
            } else {
                Some((import_path(out_path, path), name))
            }
        })
        .for_each(|(path, name)| {
            imports
                .entry(path)
                .or_insert_with(BTreeSet::<_>::new)
                .insert(name);
        });
}

pub fn import_path(from: &Path, import: &Path) -> String {
    let rel_path =
        diff_paths(import, from.parent().unwrap()).expect("failed to calculate import path");
    match rel_path.components().next() {
        Some(Component::Normal(_)) => format!("./{}", rel_path.to_string_lossy()),
        _ => rel_path.to_string_lossy().into(),
    }
    .trim_end_matches(".ts")
    .to_owned()
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
fn diff_paths<P, B>(path: P, base: B) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = base.as_ref();

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
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
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
