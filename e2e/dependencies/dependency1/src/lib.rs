use ts_rs::TS;

#[derive(TS)]
pub struct LibraryType1 {
    pub a: i32
}

#[derive(TS)]
pub struct LibraryType2<T> {
    pub t: T
}
