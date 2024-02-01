use ts_rs::TS;

#[derive(TS)]
pub struct Crate2 {
    pub x: [[[i32; 128]; 128]; 128],
}
