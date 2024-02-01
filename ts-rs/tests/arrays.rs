use ts_rs::TS;

#[test]
fn free() {
    assert_eq!(<[String; 4]>::inline(), "[string, string, string, string]")
}

#[test]
fn interface() {
    #[derive(TS)]
    struct Interface {
        #[allow(dead_code)]
        a: [i32; 4],
    }

    assert_eq!(
        Interface::inline(),
        "{ a: [number, number, number, number], }"
    )
}

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] [i32; 4]);

    assert_eq!(Newtype::inline(), "[number, number, number, number]")
}
