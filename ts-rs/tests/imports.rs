#![allow(dead_code)]
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[ts(export_to = "/tmp/ts_rs_test_type_a.ts")]
pub struct TestTypeA<T> {
    value: T,
}

#[derive(TS)]
#[ts(export)]
#[ts(export_to = "/tmp/ts_rs_test_type_b.ts")]
pub struct TestTypeB<T> {
    value: T,
}

#[derive(TS)]
#[ts(export_to = "/tmp/ts_rs_test_enum.ts")]
pub enum TestEnum {
    C { value: TestTypeB<i8> },
    A1 { value: TestTypeA<i32> },
    A2 { value: TestTypeA<i8> },
}

#[test]
#[cfg(feature = "format")]
fn test_def() {
    // The only way to get access to how the imports look is to export the type and load the exported file
    TestEnum::export().unwrap();
    let text = String::from_utf8(std::fs::read(TestEnum::EXPORT_TO.unwrap()).unwrap()).unwrap();

    // Checks to make sure imports are ordered and deduplicated
    assert_eq!(
        text,
"\
import type { TestTypeA } from \"./ts_rs_test_type_a\";
import type { TestTypeB } from \"./ts_rs_test_type_b\";

export type TestEnum = { C: { value: TestTypeB<number> } } | {
  A1: { value: TestTypeA<number> };
} | { A2: { value: TestTypeA<number> } };
"
    );

    std::fs::remove_file(TestEnum::EXPORT_TO.unwrap()).unwrap();
}
