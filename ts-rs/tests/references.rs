use ts_rs::TS;

#[test]
fn references() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct FullOfRefs<'a> {
        str_slice: &'a str,
        ref_slice: &'a [&'a str],
        num_ref: &'a i32,
    }

    assert_eq!(
        FullOfRefs::inline(),
        "{ str_slice: string, ref_slice: Array<string>, num_ref: number, }"
    )
}
