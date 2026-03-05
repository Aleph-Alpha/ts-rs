// we want to implement TS for deprecated types as well
#![allow(deprecated)]

use chrono::{
    Date, DateTime, Duration, FixedOffset, Local, Month, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Utc, Weekday,
};

use super::{impl_primitives, Config, TS};

macro_rules! impl_dummy {
    ($($t:ty),*) => {$(
        impl TS for $t {
            type WithoutGenerics = $t;
            type OptionInnerType = Self;

            fn name(_: &Config) -> String { String::new() }
            fn inline(_: &Config) -> String { String::new() }
        }
    )*};
}

impl_primitives!(NaiveDateTime, NaiveDate, NaiveTime, Month, Weekday => "string");
impl_primitives!(Duration => "[number, number]");
impl_dummy!(Utc, Local, FixedOffset);

impl<T: TimeZone + 'static> TS for DateTime<T> {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn ident(_: &Config) -> String {
        "string".to_owned()
    }
    fn name(_: &Config) -> String {
        "string".to_owned()
    }
    fn inline(_: &Config) -> String {
        "string".to_owned()
    }
}

impl<T: TimeZone + 'static> TS for Date<T> {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn ident(_: &Config) -> String {
        "string".to_owned()
    }
    fn name(_: &Config) -> String {
        "string".to_owned()
    }
    fn inline(_: &Config) -> String {
        "string".to_owned()
    }
}
