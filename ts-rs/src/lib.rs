//! Generate TypeScript interface/type declarations from rust structs.
//!
//! ## why?
//! When building a web application in rust, data structures have to be shared between backend and frontend.  
//! Using this library, you can easily generate TypeScript bindings to your rust structs & enums, so that you can keep your
//! types in one place.
//!
//! ts-rs might also come in handy when working with webassembly.
//!
//! ## how?
//! ts-rs exposes a single trait, `TS`.  
//! Using a derive macro, you can implement this trait for
//! your types.  
//! Then, you can use this trait to obtain the TypeScript bindings.
//! We recommend doing this in your tests. [see the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs)
//!
//! ## serde compatibility layer
//! With the `serde-compat` feature enabled, ts-rs tries parsing serde attributes.  
//! Please note that not all serde attributes are supported yet.

use std::path::Path;
use std::{collections::HashMap, fs::OpenOptions};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    io::{BufWriter, Write},
};

use std::any::TypeId;
pub use ts_rs_macros::TS;

#[doc(hidden)]
pub mod export;

/// A type which can be represented in TypeScript.  
/// Most of the time, you'd want to derive this trait instead of implementing it manually.  
/// ts-rs comes with implementations for all numeric types, `String`, `Vec`, `Option` and tuples.
///
/// ## get started
/// [TS](TS) can easily be derived for structs and enums:
/// ```rust
/// use ts_rs::TS;
///
/// #[derive(TS)]
/// struct User {
///     first_name: String,
///     last_name: String,
/// }
/// ```
/// To actually obtain the bindings, you can call `User::dump` to write the bindings to a file.
/// ```rust
/// # use ts_rs::TS;
/// # #[derive(TS)]
/// # struct User {
/// #     first_name: String,
/// #     last_name: String,
/// # }
/// std::fs::remove_file("bindings.ts").ok();
/// User::dump("bindings.ts").unwrap();
/// ```
///
/// Preferrably, you should use the [export!](export!) macro, which takes care of dependencies
/// between types and allows you to decide between `export` and `declare`.
///
/// ### struct attributes
///
/// - `#[ts(rename = "..")]`:  
///   Set the name of the generated interface  
///
/// - `#[ts(rename_all = "..")]`:  
///   Rename all fields of this struct.  
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`
///   
/// ### struct field attributes
///
/// - `#[ts(type = "..")]`:  
///   Overrides the type used in TypeScript  
///
/// - `#[ts(rename = "..")]`:  
///   Renames this field  
///
/// - `#[ts(inline)]`:  
///   Inlines the type of this field  
///
/// - `#[ts(skip)]`:  
///   Skip this field  
///
/// - `#[ts(flatten)]`:  
///   Flatten this field (only works if the field is a struct)  
///   
/// ### enum attributes
///
/// - `#[ts(rename = "..")]`:  
///   Set the name of the generated type  
///
/// - `#[ts(rename_all = "..")]`:  
///   Rename all variants of this enum.  
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`
///  
/// ### enum variant attributes
///
/// - `#[ts(rename = "..")]`:  
///   Renames this variant  
///
/// - `#[ts(skip)]`:  
///   Skip this variant  

pub trait TS: 'static {
    /// Declaration of this type, e.g. `interface User { user_id: number, ... }`.
    /// This function will panic if the type has no declaration.
    fn decl() -> String {
        panic!("{} cannot be declared", Self::name());
    }

    /// Name of this type in TypeScript.
    fn name() -> String;

    /// Name of this type in TypeScript, with type arguments.
    fn name_with_type_args(args: Vec<String>) -> String {
        format!("{}<{}>", Self::name(), args.join(", "))
    }

    /// Formats this types definition in TypeScript, e.g `{ user_id: number }`.
    /// This function will panic if the type cannot be inlined.
    fn inline(#[allow(unused_variables)] indent: usize) -> String {
        panic!("{} cannot be inlined", Self::name());
    }

    /// Flatten an type declaration.  
    /// This function will panic if the type cannot be flattened.
    fn inline_flattened(#[allow(unused_variables)] indent: usize) -> String {
        panic!("{} cannot be flattened", Self::name())
    }

    /// All type ids and typescript names of the types this type depends on.  
    /// This is used for resolving imports when using the `export!` macro.  
    fn dependencies() -> Vec<(TypeId, String)>;

    /// `true` if this is a transparent type, e.g tuples or a list.  
    /// This is used for resolving imports when using the `export!` macro.
    fn transparent() -> bool;

    /// Dumps the declaration of this type to a file.  
    /// If the file does not exist, it will be created.  
    /// If it does, the declaration will be appended.
    ///
    /// This function will panicked when called on a type which does not have a declaration.
    fn dump(out: impl AsRef<Path>) -> std::io::Result<()> {
        let out = out.as_ref();
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .truncate(false)
            .open(out)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(Self::decl().as_bytes())?;
        writer.write_all(b"\n\n")?;
        writer.flush()?;
        Ok(())
    }
}

macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:literal),*) => { $($(
        impl TS for $ty {
            fn name() -> String {
                $l.to_owned()
            }
            fn inline(_: usize) -> String {
                $l.to_owned()
            }
            fn dependencies() -> Vec<(TypeId, String)> {
                vec![]
            }
            fn transparent() -> bool {
                false
            }
        }
    )*)* };
}

macro_rules! impl_tuples {
    ( impl $($i:ident),* ) => {
        impl<$($i: TS),*> TS for ($($i,)*) {
            fn name() -> String {
                format!(
                    "[{}]",
                    vec![$($i::name()),*].join(", ")
                )
            }
            fn inline(indent: usize) -> String {
                format!(
                    "[{}]",
                    vec![
                        $($i::inline(indent)),*
                    ].join(", ")
                )
            }
            fn dependencies() -> Vec<(TypeId, String)> {
                vec![$((TypeId::of::<$i>(), $i::name())),*]
            }
            fn transparent() -> bool {
                true
            }
        }
    };
    ( $i2:ident $(, $i:ident)* ) => {
        impl_tuples!(impl $i2 $(, $i)* );
        impl_tuples!($($i),*);
    };
    () => {};
}

macro_rules! impl_proxy {
    ($($t:tt)*) => {
        $($t)* {
            fn name() -> String {
                T::name()
            }
            fn inline(indent: usize) -> String {
                T::inline(indent)
            }
            fn inline_flattened(indent: usize) -> String {
                T::inline_flattened(indent)
            }
            fn dependencies() -> Vec<(TypeId, String)> {
                T::dependencies()
            }
            fn transparent() -> bool {
                true
            }
        }
    };
}

impl_primitives! {
    u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize => "number",
    u128, i128 => "bigint",
    bool => "boolean",
    String, &'static str => "string",
    () => "null"
}
impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_proxy!(impl<T: TS> TS for Box<T>);
impl_proxy!(impl<T: TS> TS for std::sync::Arc<T>);
impl_proxy!(impl<T: TS> TS for std::rc::Rc<T>);
impl_proxy!(impl<T: TS + ToOwned> TS for std::borrow::Cow<'static, T>);
impl_proxy!(impl<T: TS> TS for std::cell::Cell<T>);
impl_proxy!(impl<T: TS> TS for std::cell::RefCell<T>);

impl<T: TS> TS for Option<T> {
    fn name() -> String {
        format!("{} | null", T::name())
    }

    fn inline(indent: usize) -> String {
        format!("{} | null", T::inline(indent))
    }

    fn dependencies() -> Vec<(TypeId, String)> {
        vec![(TypeId::of::<T>(), T::name())]
    }

    fn transparent() -> bool {
        true
    }
}

impl<T: TS> TS for Vec<T> {
    fn name() -> String {
        "Array".to_owned()
    }

    fn inline(indent: usize) -> String {
        format!("Array<{}>", T::inline(indent))
    }

    fn dependencies() -> Vec<(TypeId, String)> {
        vec![(TypeId::of::<T>(), T::name())]
    }

    fn transparent() -> bool {
        true
    }
}

impl<T: TS> TS for HashSet<T> {
    fn name() -> String {
        "Array".to_owned()
    }

    fn inline(indent: usize) -> String {
        format!("Array<{}>", T::inline(indent))
    }

    fn dependencies() -> Vec<(TypeId, String)> {
        vec![(TypeId::of::<T>(), T::name())]
    }

    fn transparent() -> bool {
        true
    }
}

impl<T: TS> TS for BTreeSet<T> {
    fn name() -> String {
        "Array".to_owned()
    }

    fn inline(indent: usize) -> String {
        format!("Array<{}>", T::inline(indent))
    }

    fn dependencies() -> Vec<(TypeId, String)> {
        vec![(TypeId::of::<T>(), T::name())]
    }

    fn transparent() -> bool {
        true
    }
}

impl<K: TS, V: TS> TS for HashMap<K, V> {
    fn name() -> String {
        "Record".to_owned()
    }

    fn inline(indent: usize) -> String {
        format!("Record<{}, {}>", K::inline(indent), V::inline(indent))
    }

    fn dependencies() -> Vec<(TypeId, String)> {
        vec![
            (TypeId::of::<K>(), K::name()),
            (TypeId::of::<V>(), V::name()),
        ]
    }

    fn transparent() -> bool {
        true
    }
}

impl<K: TS, V: TS> TS for BTreeMap<K, V> {
    fn name() -> String {
        "Record".to_owned()
    }

    fn inline(indent: usize) -> String {
        format!("Record<{}, {}>", K::inline(indent), V::inline(indent))
    }

    fn dependencies() -> Vec<(TypeId, String)> {
        vec![
            (TypeId::of::<K>(), K::name()),
            (TypeId::of::<V>(), V::name()),
        ]
    }

    fn transparent() -> bool {
        true
    }
}
