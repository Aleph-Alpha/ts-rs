#![allow(dead_code, clippy::blacklisted_name)]

use std::{collections::BTreeSet, rc::Rc};

use chrono::NaiveDateTime;
use serde::Serialize;
use ts_rs::{export, TS};
use uuid::Uuid;

#[derive(Serialize, TS)]
#[ts(rename_all = "lowercase")]
enum Role {
    User,
    #[ts(rename = "administrator")]
    Admin,
}

#[derive(Serialize, TS)]
// when 'serde-compat' is enabled, ts-rs tries to use supported serde attributes.
#[serde(rename_all = "UPPERCASE")]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, TS)]
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
enum Vehicle {
    Bicycle { color: String },
    Car { brand: String, color: String },
}

#[derive(Serialize, TS)]
struct Point<T>
where
    T: TS,
{
    time: u64,
    value: T,
}

#[derive(Serialize, TS)]
struct Series {
    points: Vec<Point<u64>>,
}

#[derive(Serialize, TS)]
#[serde(tag = "kind", content = "d")]
enum SimpleEnum {
    A,
    B,
}

#[derive(Serialize, TS)]
#[serde(tag = "kind", content = "data")]
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
struct ComplexStruct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_tree: Option<Rc<BTreeSet<String>>>,
}

// this will export [Role] to `role.ts` and [User] to `user.ts` when running `cargo test`.
// `export!` will also take care of including imports in typescript files.
export! {
    Role => "role.ts",
    User => "user.ts",
    // any type can be used here in place of the generic, but it has to match the one used
    // in other structs to generate the dependencies correctly:
    Point<u64> => "point.ts",
    Series => "series.ts",
    Vehicle => "vehicle.ts",
    ComplexEnum => "complex_enum.ts",
    InlineComplexEnum => "inline_complex_enum.ts",
    SimpleEnum => "simple_enum.ts",
    ComplexStruct => "complex_struct.ts",
    // this exports an ambient declaration (`declare interface`) instead of an `export interface`.
    (declare) Gender => "gender.d.ts",
}
