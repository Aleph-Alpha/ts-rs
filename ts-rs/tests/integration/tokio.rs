#![cfg(feature = "tokio-impl")]

use tokio::sync::{Mutex, OnceCell, RwLock};
use ts_rs::{Config, TS};

#[derive(TS)]
#[ts(export, export_to = "tokio/")]
#[ts(concrete(T = i32))]
struct Tokio<T: 'static> {
    mutex: Mutex<T>,
    once_cell: OnceCell<T>,
    rw_lock: RwLock<T>,
}

#[test]
fn tokio() {
    let cfg = Config::from_env();
    assert_eq!(
        Tokio::<String>::decl(&cfg),
        "type Tokio = { mutex: number, once_cell: number, rw_lock: number, };"
    )
}
