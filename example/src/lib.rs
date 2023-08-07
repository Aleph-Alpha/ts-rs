#![allow(dead_code, clippy::disallowed_names)]

use std::{collections::BTreeSet, rc::Rc};

use chrono::NaiveDateTime;
use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Serialize, TS)]
#[ts(rename_all = "lowercase")]
#[ts(export, export_to = "bindings/UserRole.ts")]
enum Role {
    User,
    #[ts(rename = "administrator")]
    Admin,
}

#[derive(Serialize, TS)]
// when 'serde-compat' is enabled, ts-rs tries to use supported serde attributes.
#[serde(rename_all = "UPPERCASE")]
#[ts(export)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, TS)]
#[ts(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    role: Role,
    family: Vec<User>,
    #[ts(inline)]
    gender: Gender,
    token: Uuid,
    #[ts(type = "string")]
    created_at: NaiveDateTime,
}

#[derive(Serialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
enum Vehicle {
    Bicycle { color: String },
    Car { brand: String, color: String },
}

#[derive(Serialize, TS)]
#[ts(export)]
struct Point<T>
where
    T: TS,
{
    time: u64,
    value: T,
}

#[derive(Serialize, TS)]
#[serde(default)]
#[ts(export)]
struct Series {
    points: Vec<Point<u64>>,
}

#[derive(Serialize, TS)]
#[serde(tag = "kind", content = "d")]
#[ts(export)]
enum SimpleEnum {
    A,
    B,
}

#[derive(Serialize, TS)]
#[serde(tag = "kind", content = "data")]
#[ts(export)]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    V(Vec<Series>),
    U(Box<User>),
}

#[derive(Serialize, TS)]
#[serde(tag = "kind")]
#[ts(export)]
enum InlineComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    V(Vec<Series>),
    U(Box<User>),
}

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct ComplexStruct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_tree: Option<Rc<BTreeSet<String>>>,
}
