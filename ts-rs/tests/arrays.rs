use ts_rs::TS;

#[test]
fn free() {
    assert_eq!(<[String; 10]>::inline(), "Array<string>")
}

#[test]
fn interface() {
    #[derive(TS)]
    struct Interface {
        #[allow(dead_code)]
        a: [i32; 10],
    }

    assert_eq!(Interface::inline(), "{ a: Array<number>, }")
}

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] [i32; 10]);

    assert_eq!(Newtype::inline(), "Array<number>")
}
