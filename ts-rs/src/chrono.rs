use chrono::{
    Date, DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
    Utc,
};

use super::{impl_primitives, TS};
use crate::Dependency;

macro_rules! impl_dummy {
    ($($t:ty),*) => {$(
        impl TS for $t {
            fn name() -> String { String::new() }
            fn inline() -> String { String::new() }
            fn dependencies() -> Vec<Dependency> { vec![] }
            fn transparent() -> bool { false }
        }
    )*};
}

impl_primitives!(NaiveDateTime, NaiveDate, NaiveTime, Duration => "string");
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
    fn name_with_type_args(_: Vec<String>) -> String {
        Self::name()
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
