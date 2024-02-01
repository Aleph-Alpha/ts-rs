use ts_rs::TS;

#[derive(TS)]
pub struct Crate1 {
    pub x: [[[i32; 128]; 128]; 128],
}