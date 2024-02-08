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
            fn name() -> String { String::new() }
            fn inline() -> String { String::new() }
            fn transparent() -> bool { false }
        }
    )*};
}

impl_primitives!(NaiveDateTime, NaiveDate, NaiveTime, Month, Weekday, Duration => "string");
impl_dummy!(Utc, Local, FixedOffset);

impl<T: TimeZone + 'static> TS for DateTime<T> {
    fn name() -> String {
        "string".to_owned()
    }
    fn name_with_type_args(_: Vec<String>) -> String {
        Self::name()
    }
    fn inline() -> String {
        "string".to_owned()
    }
    fn transparent() -> bool {
        false
    }
}

impl<T: TimeZone + 'static> TS for Date<T> {
    fn name() -> String {
        "string".to_owned()
    }
    fn name_with_type_args(_: Vec<String>) -> String {
        Self::name()
    }
    fn inline() -> String {
        "string".to_owned()
    }
    fn transparent() -> bool {
        false
    }
}
