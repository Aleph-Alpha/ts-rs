#![cfg(feature = "chrono-impl")]

use chrono::{
    Date, DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc,
};
use ts_rs::TS;

#[test]
fn chrono() {
    #[derive(TS)]
    #[allow(dead_code)]
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
    }

    assert_eq!(
        Chrono::decl(),
        "interface Chrono { date: [string, string, string, string], time: string, date_time: [string, string, string, string], duration: string, }"
    )
}
