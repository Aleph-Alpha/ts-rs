#![cfg(feature = "semver-impl")]

use semver::Version;
use ts_rs::TS;

#[test]
fn semver() {
    #[derive(TS)]
    struct Semver {
        #[allow(dead_code)]
        version: Version,
    }

    assert_eq!(Semver::decl(), "type Semver = { version: string, }")
}
