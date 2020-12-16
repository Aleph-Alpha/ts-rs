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
//! Using a derive macro, you can implement this interface for
//! your types.  
//! Then, you can use this trait to obtain the TypeScript bindings.
//! We recommend doing this in your tests. [see the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs)
//!
//! ## serde compatibility layer
//! With the `serde-compat` feature enabled, ts-rs tries parsing serde attributes.  
//! Please note that not all serde attributes are supported yet.

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub use ts_rs_macros::TS;

/// A type which can be represented in TypeScript.  
/// Most of the time, you'd want to derive this trait instead of implementing it manually.  
/// ts-rs comes with implementations for all numeric types, `String`, `Vec`, `Option` and tuples.
///
/// ## get started
/// [TS](ts_rs::TS) can easily be derived for structs and enums:
/// ```rust
/// use ts_rs::TS;
///
/// #[derive(TS)]
/// struct User {
///     first_name: String,
///     last_name: String,
/// }
/// ```
/// To actually obtain the bindings, you can call `User::dump` to write the bindings to a file:
/// ```rust
/// # use ts_rs::TS;
///
/// # #[derive(TS)]
/// # struct User {
/// #     first_name: String,
/// #     last_name: String,
/// # }
/// std::fs::remove_file("bindings.ts").ok();
/// User::dump("bindings.ts").unwrap();
/// ```
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

pub trait TS {
    /// Declaration of this type, e.g. `interface User { user_id: number, ... }`, if available.
    fn decl() -> Option<String> {
        None
    }

    /// Formats this type.
    /// When using inline, this will return the definition of the type.
    /// Otherwise, it's name is returned (if the type is named)
    // TODO: split this into `name(indent)` and `format(indent)`
    fn format(indent: usize, inline: bool) -> String;
    
    /// Flatten an interface declaration.  
    /// This will panic if this is not an interface.
    fn flatten_interface(#[allow(unused_variables)] indent: usize) -> String {
        panic!("this type cannot be inlined!")
    }

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
        writer.write_all(Self::decl().expect("Type has no declaration").as_bytes())?;
        writer.write_all(b"\n\n")?;
        writer.flush()?;
        Ok(())
    }
}

macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:literal),*) => { $($(
        impl TS for $ty {
            fn decl() -> Option<String> { None }
            fn format(_: usize, _: bool) -> String {
                $l.to_owned()
            }
        }
    )*)* };
}

macro_rules! impl_tuples {
    ( impl $($i:ident),* ) => {
        impl<$($i: TS),*> TS for ($($i,)*) {
            fn format(indent: usize, inline: bool) -> String {
                format!(
                    "[{}]",
                    vec![$($i::format(indent, inline)),*].join(", ")
                )
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
            fn format(indent: usize, inline: bool) -> String { T::format(indent, inline) }
        }
    };
}

impl_primitives! {
    u8, i8, u16, i16, u32, i32, u64, i64, f32, f64 => "number",
    u128, i128 => "bigint",
    bool => "boolean",
    String, &str => "string"
}
impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_proxy!(impl<T: TS> TS for &T);
impl_proxy!(impl<T: TS> TS for Box<T>);
impl_proxy!(impl<T: TS> TS for std::sync::Arc<T>);
impl_proxy!(impl<T: TS> TS for std::rc::Rc<T>);
impl_proxy!(impl<'a, T: TS + ToOwned> TS for std::borrow::Cow<'a, T>);
impl_proxy!(impl<T: TS> TS for std::cell::Cell<T>);
impl_proxy!(impl<T: TS> TS for std::cell::RefCell<T>);

impl<T: TS> TS for Option<T> {
    fn decl() -> Option<String> {
        None
    }

    fn format(indent: usize, inline: bool) -> String {
        format!("{} | null", T::format(indent, inline))
    }
}

impl<T: TS> TS for Vec<T> {
    fn format(indent: usize, inline: bool) -> String {
        format!("{}[]", T::format(indent, inline))
    }
}
