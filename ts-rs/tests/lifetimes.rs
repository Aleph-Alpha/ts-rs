use ts_rs::TS;

#[test]
fn contains_borrow() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct S<'a> {
        s: &'a str,
    }

    assert_eq!(S::decl(), "interface S { s: string, }")
}

#[test]
fn contains_borrow_type_args() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct B<'a, T: 'a> {
        a: &'a T,
    }

    #[derive(TS)]
    #[allow(dead_code)]
    struct A<'a> {
        a: &'a &'a &'a Vec<u32>,                        //Multiple References
        b: &'a Vec<B<'a, u32>>,                         //Nesting
        c: &'a std::collections::HashMap<String, bool>, //Multiple type args
    }

    assert_eq!(
        A::decl(),
        "interface A { a: Array<number>, b: Array<B<number>>, c: Record<string, boolean>, }"
    );
}
