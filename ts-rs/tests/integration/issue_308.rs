use ts_rs::TS;

trait Malicious {
    fn name() -> String {
        unimplemented!()
    }
    fn inline() -> String {
        unimplemented!()
    }
}

impl<T> Malicious for T {}

#[derive(TS)]
#[ts(export, export_to = "issue_308/")]
struct MyStruct<A, B>(A, B);
