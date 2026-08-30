#![cfg(feature = "astrolabe-impl")]
use astrolabe::{Date, DateTime, Time};
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "astrolabe/")]
struct Astrolabe {
    date: Date,
    time: Time,
    date_time: DateTime,
}

#[test]
fn astrolabe() {
    let cfg = Config::from_env();
    assert_eq!(
        Astrolabe::decl(&cfg),
        "type Astrolabe = { date: string, time: string, date_time: string, };"
    )
}
