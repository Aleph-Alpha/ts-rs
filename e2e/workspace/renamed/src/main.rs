use ts_renamed::TS;

#[derive(TS)]
#[ts(crate = "ts_renamed", export)]
pub struct SimpleStruct {
    hello: String,
    world: u32,
}

fn main() {
    println!("Hello, world!");
}
