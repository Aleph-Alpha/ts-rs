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

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

pub use ts_rs_macros::TS;

pub use crate::export::ExportError;

mod export;

/// A type which can be represented in TypeScript.  
/// Most of the time, you'd want to derive this trait instead of implementing it manually.  
/// ts-rs comes with implementations for all primitives, most collections, tuples,
/// arrays and containers.
///
/// ## get started
/// [TS](TS) can easily be derived for structs and enums:
/// ```rust
/// use ts_rs::TS;
///
/// #[derive(TS)]
/// #[ts(export)]
/// struct User {
///     first_name: String,
///     last_name: String,
/// }
/// ```
/// `#[ts(export)]` will generate a test for you, in which the bindings are exported.
/// After running `cargo test`, there should be a new file, `User.ts` in the `typescript/` directory.
/// This behaviour can be customized by adding `#[ts(export_to = "..")]` to the type and/or configuring
/// the output directory in `ts.toml`.
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
/// - `#[ts(optional)]
///   Indicates the field may be omitted from the serialized struct
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
#[derive(Eq, PartialEq, Ord, PartialOrd)]
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
        Some(Dependency {
            type_id: TypeId::of::<T>(),
            ts_name: T::name(),
            exported_to: T::EXPORT_TO?,
        })
    }
}

macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:literal),*) => { $($(
        impl TS for $ty {
            fn name() -> String {
                $l.to_owned()
            }
            fn inline() -> String {
                $l.to_owned()
            }
            fn dependencies() -> Vec<Dependency> {
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
            fn inline() -> String {
                format!(
                    "[{}]",
                    vec![
                        $($i::inline()),*
                    ].join(", ")
                )
            }
            fn dependencies() -> Vec<Dependency> {
                [$(
                    Dependency::from_ty::<$i>()
                ),*]
                .into_iter()
                .flatten()
                .collect()
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
            fn name_with_type_args(args: Vec<String>) -> String {
                if args.len() == 1 {
                    args[0].clone()
                } else {
                    format!("[{}]", args.join(", "))
                }
            }
            fn inline() -> String {
                T::inline()
            }
            fn inline_flattened() -> String {
                T::inline_flattened()
            }
            fn dependencies() -> Vec<Dependency> {
                T::dependencies()
            }
            fn transparent() -> bool {
                T::transparent()
            }
        }
    };
}

impl_primitives! {
    u8, i8, u16, i16, u32, i32, f32, f64, usize, isize => "number",
    u64, i64, u128, i128 => "bigint",
    bool => "boolean",
    String, &'static str => "string",
    () => "null"
}

#[cfg(feature = "bytes-impl")]
mod bytes {
    use super::TS;
    use crate::Dependency;

    impl TS for bytes::Bytes {
        fn name() -> String {
            "Array<number>".to_owned()
        }

        fn inline() -> String {
            format!("Array<{}>", u8::inline())
        }

        fn dependencies() -> Vec<Dependency> {
            vec![]
        }

        fn transparent() -> bool {
            true
        }
    }

    impl TS for bytes::BytesMut {
        fn name() -> String {
            "Array<number>".to_owned()
        }

        fn inline() -> String {
            format!("Array<{}>", u8::inline())
        }

        fn dependencies() -> Vec<Dependency> {
            vec![]
        }

        fn transparent() -> bool {
            true
        }
    }
}

#[cfg(feature = "chrono-impl")]
mod chrono_impls {
    use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

    use super::TS;
    use crate::Dependency;

    impl_primitives! {
        NaiveDateTime, NaiveDate, NaiveTime => "string"
    }

    impl<T: TimeZone + 'static> TS for DateTime<T> {
        fn name() -> String {
            "string".to_owned()
        }

        fn inline() -> String {
            "string".to_owned()
        }

        fn dependencies() -> Vec<Dependency> {
            vec![]
        }

        fn transparent() -> bool {
            false
        }
    }

    impl<T: TimeZone + 'static> TS for Date<T> {
        fn name() -> String {
            "string".to_owned()
        }

        fn inline() -> String {
            "string".to_owned()
        }

        fn dependencies() -> Vec<Dependency> {
            vec![]
        }

        fn transparent() -> bool {
            false
        }
    }
}

#[cfg(feature = "bigdecimal-impl")]
impl_primitives! {
    bigdecimal::BigDecimal => "string"
}

#[cfg(feature = "uuid-impl")]
impl_primitives! {
    uuid::Uuid => "string"
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
        unreachable!();
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert_eq!(args.len(), 1);
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
        assert_eq!(args.len(), 1);
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

impl<T: TS> TS for HashSet<T> {
    fn name() -> String {
        "Array".to_owned()
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

impl<T: TS> TS for BTreeSet<T> {
    fn name() -> String {
        "Array".to_owned()
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

impl<K: TS, V: TS> TS for BTreeMap<K, V> {
    fn name() -> String {
        "Record".to_owned()
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

impl<T: TS, const N: usize> TS for [T; N] {
    fn name() -> String {
        format!("Array<{}>", T::name())
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
