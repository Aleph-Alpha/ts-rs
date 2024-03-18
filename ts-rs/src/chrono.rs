// we want to implement TS for deprecated types as well
#![allow(deprecated)]

use chrono::{
    Date, DateTime, Duration, FixedOffset, Local, Month, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Utc, Weekday,
};

use super::{impl_primitives, TS};

macro_rules! impl_dummy {
    ($($t:ty),*) => {$(
        impl TS for $t {
            type WithoutGenerics = $t;
            fn name() -> String { String::new() }
            fn inline() -> String { String::new() }
            fn inline_flattened() -> String { panic!("{} cannot be flattened", Self::name()) }
            fn decl() -> String { panic!("{} cannot be declared", Self::name()) }
            fn decl_concrete() -> String { panic!("{} cannot be declared", Self::name()) }
        }
    )*};
}

impl_primitives!(NaiveDateTime, NaiveDate, NaiveTime, Month, Weekday, Duration => "string");
impl_dummy!(Utc, Local, FixedOffset);

impl<T: TimeZone + 'static> TS for DateTime<T> {
    type WithoutGenerics = Self;
    fn ident() -> String {
        "string".to_owned()
    }
    fn name() -> String {
        "string".to_owned()
    }
    fn inline() -> String {
        "string".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

impl<T: TimeZone + 'static> TS for Date<T> {
    type WithoutGenerics = Self;
    fn ident() -> String {
        "string".to_owned()
    }
    fn name() -> String {
        "string".to_owned()
    }
    fn inline() -> String {
        "string".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}
