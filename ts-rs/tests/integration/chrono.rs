#![allow(deprecated, dead_code)]
#![cfg(feature = "chrono-impl")]

use chrono::{
    Date, DateTime, Duration, FixedOffset, Local, Month, NaiveDate, NaiveDateTime, NaiveTime, Utc,
    Weekday,
};
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "chrono/")]
struct Chrono {
    date: (NaiveDate, Date<Utc>, Date<Local>, Date<FixedOffset>),
    time: NaiveTime,
    date_time: (
        NaiveDateTime,
        DateTime<Utc>,
        DateTime<Local>,
        DateTime<FixedOffset>,
    ),
    duration: Duration,
    month: Month,
    weekday: Weekday,
}

#[test]
fn chrono() {
    let month = Month::January;
    assert_eq!(
        Chrono::decl(),
        "type Chrono = { date: [string, Date, Date, Date], time: string, date_time: [string, Date, Date, Date], duration: string, month: number, weekday: number, };"
    )
}
