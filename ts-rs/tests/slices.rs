use ts_rs::TS;

#[test]
fn free() {
    assert_eq!(<[String]>::inline(), "Array<string>")
}

#[test]
fn interface() {
    #[derive(TS)]
    struct Interface {
        #[allow(dead_code)]
        a: [i32],
    }

    assert_eq!(Interface::inline(), "{ a: Array<number>, }")
}

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] [i32]);

    assert_eq!(Newtype::inline(), "Array<number>")
}

// Since slices usually need to be wrapped in a `Box` or other container,
// these tests should to check for that

#[test]
fn boxed_free() {
    assert_eq!(<Box<[String]>>::inline(), "Array<string>")
}

#[test]
fn boxed_interface() {
    #[derive(TS)]
    struct Interface {
        #[allow(dead_code)]
        a: Box<[i32]>,
    }

    assert_eq!(Interface::inline(), "{ a: Array<number>, }")
}

#[test]
fn boxed_newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] Box<[i32]>);

    assert_eq!(Newtype::inline(), "Array<number>")
}
