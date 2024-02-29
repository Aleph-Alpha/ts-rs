use ts_rs::TS;
use dependency1::*;

#[derive(TS)]
#[ts(export)]
struct ConsumerType {
    pub ty1: LibraryType1,
    pub ty2_1: LibraryType2<i32>,
    pub ty2_2: LibraryType2<&'static Self>,
    pub ty2_3: LibraryType2<LibraryType2<Box<ConsumerType>>>,
    pub ty2_4: LibraryType2<LibraryType2<LibraryType1>>,
    pub ty2_5: LibraryType2<LibraryType3>,
}

#[derive(TS)]
#[ts(export)]
struct T0;

#[derive(TS)]
#[ts(export)]
struct T1 {
    t0: Option<T0>
}


fn main() {}