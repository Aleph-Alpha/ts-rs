use ts_rs::TS;
use crate1::Crate1;
use crate2::Crate2;

fn main() {
    println!("Hello, world!");
}

#[derive(TS)]
#[ts(export)]
pub struct Parent {
    pub crate1: Crate1,
    pub crate2: Crate2,
}
