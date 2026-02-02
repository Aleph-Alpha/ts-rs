#![cfg(feature = "jiff-impl")]
use jiff::{
    civil::{Date, DateTime, Time},
    Span, Timestamp, Zoned,
};
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "jiff/")]
struct Jiff {
    date: Date,
    time: Time,
    date_time: DateTime,
    timestamp: Timestamp,
    span: Span,
}

#[test]
fn jiff() {
    let cfg = Config::from_env();
    assert_eq!(
        Jiff::decl(&cfg),
        "type Jiff = { date: string, time: string, date_time: string, timestamp: string, span: string, };"
    )
}
