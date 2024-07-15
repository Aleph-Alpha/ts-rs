#![cfg(feature = "bson")]

use bson::oid::ObjectId;
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "bson/")]
struct User {
    _id: ObjectId,
}

#[test]
fn bson() {
    assert_eq!(User::decl(), "type User = { _id: string, };")
}
