#![allow(dead_code)]

use ts_rs::TS;

#[derive(TS)]
enum Role {
    User,
    Admin,
}

#[derive(TS)]
#[ts(rename_all = "lowercase")]
enum Gender {
    Male,
    Female,
    Other
}

#[derive(TS)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    role: Role,
    #[ts(inline)]
    gender: Gender
}

#[cfg(test)]
mod export_ts {
    use crate::{Role, User};
    use ts_rs::TS;

    #[test]
    fn export_ts() {
        let _ = std::fs::remove_file("bindings.ts");
        Role::dump("bindings.ts").unwrap();
        User::dump("bindings.ts").unwrap();
    }
}
