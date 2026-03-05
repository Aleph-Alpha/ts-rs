#![cfg(feature = "bson-uuid-impl")]

use bson::{oid::ObjectId, Uuid};
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "bson/")]
struct User {
    _id: ObjectId,
    _uuid: Uuid,
}

#[test]
fn bson() {
    let cfg = Config::from_env();
    assert_eq!(User::decl(&cfg), "type User = { _id: string, _uuid: string, };")
}
