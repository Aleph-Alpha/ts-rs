use ts_rs::TS;

#[derive(TS)]
#[ts(export_to = "crate1/")]
pub struct Crate1 {
    pub x: [[[i32; 128]; 128]; 128],
}
