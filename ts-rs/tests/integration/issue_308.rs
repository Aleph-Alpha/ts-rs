use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "issue_308/")]
struct MyStruct<A, B>(A, B);

impl<A, B> MyStruct<A, B> {
    fn name() -> String {
        unimplemented!()
    }
}
