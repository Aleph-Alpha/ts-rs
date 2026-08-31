use astrolabe::{
    Date, DateTime, Time
};

use super::{impl_primitives, TS};

impl_primitives!(Date, DateTime, Time => "string");


