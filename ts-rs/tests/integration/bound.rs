#![allow(dead_code)]

use ts_rs::{Config, TS};

trait Driver {
    type Info;
}

struct TsDriver;

#[derive(TS)]
struct TsInfo;

impl Driver for TsDriver {
    type Info = TsInfo;
}

#[derive(TS)]
#[ts(export, export_to = "bound/")]
#[ts(concrete(D = TsDriver))]
struct Inner<D: Driver> {
    info: D::Info,
}

#[derive(TS)]
#[ts(export, export_to = "bound/")]
#[ts(concrete(D = TsDriver), bound = "D::Info: TS")]
struct Outer<D: Driver> {
    inner: Inner<D>,
}

#[test]
fn test_bound() {
    let cfg = Config::from_env();
    assert_eq!(
        Outer::<TsDriver>::decl(&cfg),
        "type Outer = { inner: Inner, };"
    );
    assert_eq!(
        Inner::<TsDriver>::decl(&cfg),
        "type Inner = { info: TsInfo, };"
    );
}
