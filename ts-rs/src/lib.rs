use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub use ts_rs_macros::TS;

pub trait TS {
    /// Declaration of this type, e.g. `interface User { user_id: number, ... }`, if available.
    fn decl() -> Option<String> {
        None
    }
    /// Formats this type.
    /// When using inline, this will return the definition of the type.
    /// Otherwise, it's name is returned (if the type is named)
    fn format(indent: usize, inline: bool) -> String;

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
