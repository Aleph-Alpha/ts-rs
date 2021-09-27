#![allow(dead_code)]

use serde::Serialize;
use ts_rs::{export, TS};
use chrono::NaiveDateTime;

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
    gender: Gender,
    created_at: NaiveDateTime,
}

#[derive(Serialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Vehicle {
    Bicycle { color: String },
    Car { brand: String, color: String },
}

// this will export [Role] to `role.ts` and [User] to `user.ts` when running `cargo test`.
// `export!` will also take care of including imports in typescript files.
export! {
    Role => "role.ts",
    User => "user.ts",
    Vehicle => "vehicle.ts",
    // this exports an ambient declaration (`declare interface`) instead of an `export interface`.
    (declare) Gender => "gender.d.ts",
}
