use ts_rs::TS;
use dependency1::LibraryType;

#[derive(TS)]
#[ts(export)]
struct ConsumerType {
    pub ty: LibraryType
}

fn main() {}