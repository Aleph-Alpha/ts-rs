//! <h1 align="center" style="padding-top: 0; margin-top: 0;">
//! <img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/ts-rs/main/logo.png" alt="logo">
//! <br/>
//! ts-rs
//! </h1>
//! <p align="center">
//! generate typescript interface/type declarations from rust types
//! </p>
//!
//! <div align="center">
//! <!-- Github Actions -->
//! <img src="https://img.shields.io/github/workflow/status/Aleph-Alpha/ts-rs/Test?style=flat-square" alt="actions status" />
//! <a href="https://crates.io/crates/ts-rs">
//! <img src="https://img.shields.io/crates/v/ts-rs.svg?style=flat-square"
//! alt="Crates.io version" />
//! </a>
//! <a href="https://docs.rs/ts-rs">
//! <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//! alt="docs.rs docs" />
//! </a>
//! <a href="https://crates.io/crates/ts-rs">
//! <img src="https://img.shields.io/crates/d/ts-rs.svg?style=flat-square"
//! alt="Download" />
//! </a>
//! </div>
//!
//! ## why?
//! When building a web application in rust, data structures have to be shared between backend and frontend.
//! Using this library, you can easily generate TypeScript bindings to your rust structs & enums so that you can keep your
//! types in one place.
//!
//! ts-rs might also come in handy when working with webassembly.
//!
//! ## how?
//! ts-rs exposes a single trait, `TS`. Using a derive macro, you can implement this interface for your types.
//! Then, you can use this trait to obtain the TypeScript bindings.
//! We recommend doing this in your tests.
//! [See the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/ts-rs/latest/ts_rs/).
//!
//! ## get started
//! ```toml
//! [dependencies]
//! ts-rs = "6.1"
//! ```
//!
//! ```rust
//! use ts_rs::TS;
//!
//! #[derive(TS)]
//! #[ts(export)]
//! struct User {
//!     user_id: i32,
//!     first_name: String,
//!     last_name: String,
//! }
//! ```
//! When running `cargo test`, the TypeScript bindings will be exported to the file `bindings/User.ts`.
//!
//! ## features
//! - generate interface declarations from rust structs
//! - generate union declarations from rust enums
//! - inline types
//! - flatten structs/interfaces
//! - generate necessary imports when exporting to multiple files
//! - serde compatibility
//! - generic types
//!
//! ## cargo features
//! - `serde-compat` (default)  
//!
//!   Enable serde compatibility. See below for more info.  
//! - `format` (default)  
//!
//!   When enabled, the generated typescript will be formatted.
//!   Currently, this sadly adds quite a bit of dependencies.
//! - `chrono-impl`  
//!
//!   Implement `TS` for types from chrono  
//! - `bigdecimal-impl`  
//!
//!   Implement `TS` for types from bigdecimal  
//! - `uuid-impl`  
//!
//!   Implement `TS` for types from uuid  
//! - `bytes-impl`  
//!
//!   Implement `TS` for types from bytes  
//!
//! If there's a type you're dealing with which doesn't implement `TS`, use `#[ts(type = "..")]` or open a PR.
//!
//! ## serde compatability
//! With the `serde-compat` feature (enabled by default), serde attributes can be parsed for enums and structs.
//! Supported serde attributes:
//! - `rename`
//! - `rename-all`
//! - `tag`
//! - `content`
//! - `untagged`
//! - `skip`
//! - `skip_serializing`
//! - `skip_deserializing`
//! - `skip_serializing_if = "Option::is_none"`
//! - `flatten`
//! - `default`
//!
//! When ts-rs encounters an unsupported serde attribute, a warning is emitted.
//!
//! ## contributing
//! Contributions are always welcome!
//! Feel free to open an issue, discuss using GitHub discussions or open a PR.
//! [See CONTRIBUTING.md](https://github.com/Aleph-Alpha/ts-rs/blob/main/CONTRIBUTING.md)
//!
//! ## todo
//! - [x] serde compatibility layer
//! - [x] documentation
//! - [x] use typescript types across files
//! - [x] more enum representations
//! - [x] generics
//! - [ ] don't require `'static`

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
};

pub use ts_rs_macros::TS;
pub use crate::export::ExportError;

#[cfg(feature = "chrono-impl")]
mod chrono;
mod export;

/// A type which can be represented in TypeScript.  
/// Most of the time, you'd want to derive this trait instead of implementing it manually.  
/// ts-rs comes with implementations for all primitives, most collections, tuples,
/// arrays and containers.
///
/// ### exporting
/// Because Rusts procedural macros are evaluated before other compilation steps, TypeScript
/// bindings cannot be exported during compile time.
/// Bindings can be exported within a test, which ts-rs generates for you by adding `#[ts(export)]`
/// to a type you wish to export to a file.
/// If, for some reason, you need to do this during runtime, you can call [`TS::export`] yourself.
///
/// ### serde compatibility
/// By default, the feature `serde-compat` is enabled.
/// ts-rs then parses serde attributes and adjusts the generated typescript bindings accordingly.
/// Not all serde attributes are supported yet - if you use an unsupported attribute, you'll see a
/// warning.
///
/// ### container attributes
/// attributes applicable for both structs and enums
///
/// - `#[ts(export)]`:  
///   Generates a test which will export the type, by default to `bindings/<name>.ts` when running
///   `cargo test`
///
/// - `#[ts(export_to = "..")]`:  
///   Specifies where the type should be exported to. Defaults to `bindings/<name>.ts`.
///
/// - `#[ts(rename = "..")]`:  
///   Sets the typescript name of the generated type
///
/// - `#[ts(rename_all = "..")]`:  
///   Rename all fields/variants of the type.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`
///
///
/// ### struct field attributes
///
/// - `#[ts(type = "..")]`:  
///   Overrides the type used in TypeScript.  
///   This is useful when there's a type for which you cannot derive `TS`.  
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
/// - `#[ts(optional)]`:  
///   Indicates the field may be omitted from the serialized struct
///
/// - `#[ts(flatten)]`:  
///   Flatten this field (only works if the field is a struct)  
///   
/// ### enum attributes
///
/// - `#[ts(tag = "..")]`:  
///   Changes the representation of the enum to store its tag in a separate field.
///   See [the serde docs](https://serde.rs/enum-representations.html).
///
/// - `#[ts(content = "..")]`:  
///   Changes the representation of the enum to store its content in a separate field.
///   See [the serde docs](https://serde.rs/enum-representations.html).
///
/// - `#[ts(untagged)]`:  
///   Changes the representation of the enum to not include its tag.
///   See [the serde docs](https://serde.rs/enum-representations.html).
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
    const EXPORT_TO: Option<&'static str> = None;

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
    fn inline() -> String {
        panic!("{} cannot be inlined", Self::name());
    }

    /// Flatten an type declaration.  
    /// This function will panic if the type cannot be flattened.
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }

    /// Information about types this type depends on.
    /// This is used for resolving imports when exporting to a file.
    fn dependencies() -> Vec<Dependency>;

    /// `true` if this is a transparent type, e.g tuples or a list.  
    /// This is used for resolving imports when using the `export!` macro.
    fn transparent() -> bool;

    /// Manually export this type to a file.
    /// The output file can be specified by annotating the type with `#[ts(export_to = ".."]`.
    /// By default, the filename will be derived from the types name.
    ///
    /// When a type is annotated with `#[ts(export)]`, it is exported automatically within a test.
    /// This function is only usefull if you need to export the type outside of the context of a
    /// test.
    fn export() -> Result<(), ExportError> {
        export::export_type::<Self>()
    }
}

/// A typescript type which is depended upon by other types.
/// This information is required for generating the correct import statements.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency {
    /// Type ID of the rust type
    pub type_id: TypeId,
    /// Name of the type in TypeScript
    pub ts_name: String,
    /// Path to where the type would be exported. By default a filename is derived from the types
    /// name, which can be customized with `#[ts(export_to = "..")]`.
    pub exported_to: &'static str,
}

impl Dependency {
    /// Constructs a [`Dependency`] from the given type `T`.
    /// If `T` is not exportable (meaning `T::EXPORT_TO` is `None`), this function will return
    /// `None`
    pub fn from_ty<T: TS>() -> Option<Self> {
        let exported_to = T::EXPORT_TO?;
        Some(Dependency {
            type_id: TypeId::of::<T>(),
            ts_name: T::name(),
            exported_to,
        })
    }
}

// generate impls for primitive types
macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:literal),*) => { $($(
        impl TS for $ty {
            fn name() -> String { $l.to_owned() }
            fn name_with_type_args(args: Vec<String>) -> String {
                assert!(args.is_empty(), "called name_with_type_args on primitive");
                $l.to_owned()
            }
            fn inline() -> String { $l.to_owned() }
            fn dependencies() -> Vec<Dependency> { vec![] }
            fn transparent() -> bool { false }
        }
    )*)* };
}
// generate impls for tuples
macro_rules! impl_tuples {
    ( impl $($i:ident),* ) => {
        impl<$($i: TS),*> TS for ($($i,)*) {
            fn name() -> String {
                format!("[{}]", vec![$($i::name()),*].join(", "))
            }
            fn inline() -> String {
                format!("[{}]", vec![ $($i::inline()),* ].join(", "))
            }
            fn dependencies() -> Vec<Dependency> {
                [$( Dependency::from_ty::<$i>() ),*]
                .into_iter()
                .flatten()
                .collect()
            }
            fn transparent() -> bool { true }
        }
    };
    ( $i2:ident $(, $i:ident)* ) => {
        impl_tuples!(impl $i2 $(, $i)* );
        impl_tuples!($($i),*);
    };
    () => {};
}

// generate impls for wrapper types
macro_rules! impl_wrapper {
    ($($t:tt)*) => {
        $($t)* {
            fn name() -> String { T::name() }
            fn name_with_type_args(mut args: Vec<String>) -> String {
                assert_eq!(args.len(), 1);
                args.remove(0)
            }
            fn inline() -> String { T::inline() }
            fn inline_flattened() -> String { T::inline_flattened() }
            fn dependencies() -> Vec<Dependency> { T::dependencies() }
            fn transparent() -> bool { T::transparent() }
        }
    };
}

// implement TS for the $shadow, deferring to the impl $s
macro_rules! impl_shadow {
    (as $s:ty: $($impl:tt)*) => {
        $($impl)* {
            fn name() -> String { <$s>::name() }
            fn name_with_type_args(args: Vec<String>) -> String { <$s>::name_with_type_args(args) }
            fn inline() -> String { <$s>::inline() }
            fn inline_flattened() -> String { <$s>::inline_flattened() }
            fn dependencies() -> Vec<$crate::Dependency> { <$s>::dependencies() }
            fn transparent() -> bool { <$s>::transparent() }
        }
    };
}

impl<T: TS> TS for Option<T> {
    fn name() -> String {
        unreachable!();
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called Option::name_with_type_args with {} args",
            args.len()
        );
        format!("{} | null", args[0])
    }

    fn inline() -> String {
        format!("{} | null", T::inline())
    }

    fn dependencies() -> Vec<Dependency> {
        [Dependency::from_ty::<T>()].into_iter().flatten().collect()
    }

    fn transparent() -> bool {
        true
    }
}

impl<T: TS> TS for Vec<T> {
    fn name() -> String {
        "Array".to_owned()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called Vec::name_with_type_args with {} args",
            args.len()
        );
        format!("Array<{}>", args[0])
    }

    fn inline() -> String {
        format!("Array<{}>", T::inline())
    }

    fn dependencies() -> Vec<Dependency> {
        [Dependency::from_ty::<T>()].into_iter().flatten().collect()
    }

    fn transparent() -> bool {
        true
    }
}

impl<K: TS, V: TS> TS for HashMap<K, V> {
    fn name() -> String {
        "Record".to_owned()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            2,
            "called HashMap::name_with_type_args with {} args",
            args.len()
        );
        format!("Record<{}, {}>", args[0], args[1])
    }

    fn inline() -> String {
        format!("Record<{}, {}>", K::inline(), V::inline())
    }

    fn dependencies() -> Vec<Dependency> {
        [Dependency::from_ty::<K>(), Dependency::from_ty::<V>()]
            .into_iter()
            .flatten()
            .collect()
    }

    fn transparent() -> bool {
        true
    }
}

impl<I: TS> TS for Range<I> {
    fn name() -> String {
        panic!("called Range::name - Did you use a type alias?")
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called Range::name_with_type_args with {} args",
            args.len()
        );
        format!("{{ start: {}, end: {}, }}", &args[0], &args[0])
    }

    fn dependencies() -> Vec<Dependency> {
        [Dependency::from_ty::<I>()].into_iter().flatten().collect()
    }

    fn transparent() -> bool {
        true
    }
}

impl<I: TS> TS for RangeInclusive<I> {
    fn name() -> String {
        panic!("called RangeInclusive::name - Did you use a type alias?")
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(
            args.len(),
            1,
            "called RangeInclusive::name_with_type_args with {} args",
            args.len()
        );
        format!("{{ start: {}, end: {}, }}", &args[0], &args[0])
    }

    fn dependencies() -> Vec<Dependency> {
        [Dependency::from_ty::<I>()].into_iter().flatten().collect()
    }

    fn transparent() -> bool {
        true
    }
}

impl_shadow!(as Vec<T>: impl<T: TS> TS for HashSet<T>);
impl_shadow!(as Vec<T>: impl<T: TS> TS for BTreeSet<T>);
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for BTreeMap<K, V>);
impl_shadow!(as Vec<T>: impl<T: TS, const N: usize> TS for [T; N]);

impl_wrapper!(impl<T: TS> TS for Box<T>);
impl_wrapper!(impl<T: TS> TS for std::sync::Arc<T>);
impl_wrapper!(impl<T: TS> TS for std::rc::Rc<T>);
impl_wrapper!(impl<T: TS + ToOwned> TS for std::borrow::Cow<'static, T>);
impl_wrapper!(impl<T: TS> TS for std::cell::Cell<T>);
impl_wrapper!(impl<T: TS> TS for std::cell::RefCell<T>);

impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

#[cfg(feature = "bigdecimal-impl")]
impl_primitives! { bigdecimal::BigDecimal => "string" }

#[cfg(feature = "uuid-impl")]
impl_primitives! { uuid::Uuid => "string" }

#[cfg(feature = "bytes-impl")]
mod bytes {
    use super::TS;

    impl_shadow!(as Vec<u8>: impl TS for bytes::Bytes);
    impl_shadow!(as Vec<u8>: impl TS for bytes::BytesMut);
}

impl_primitives! {
    u8, i8, u16, i16, u32, i32, f32, f64, usize, isize => "number",
    u64, i64, u128, i128 => "bigint",
    bool => "boolean",
    Path, PathBuf, String, &'static str => "string",
    () => "null"
}
#[rustfmt::skip]
pub(crate) use impl_primitives;
