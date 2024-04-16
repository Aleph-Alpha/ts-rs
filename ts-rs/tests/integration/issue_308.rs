use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "issue_308/")]
struct MyStruct;

impl MyStruct {
    fn name() -> String {
        unimplemented!()
    }
}
