use ts_rs::TS;
use std::time::Duration;

#[test]
fn time() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct Time {
        duration: Duration
    }

    assert_eq!(
        Time::decl(),
        "interface Time { duration: string, }"
    )
}