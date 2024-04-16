use ts_rs::TS;

trait Malicious {
    fn name() -> String;
}

impl<T> Malicious for T {
    fn name() -> String {
        unimplemented!()
    }
}

#[derive(TS)]
#[ts(export, export_to = "issue_308/")]
struct MyStruct<A, B>(A, B);