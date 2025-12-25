use jiff::{
    civil::{Date, DateTime, Time},
    Span, Timestamp, Zoned,
};

use super::{impl_primitives, TS};

impl_primitives!(Date, DateTime, Span, Time, Timestamp, Zoned => "string");
