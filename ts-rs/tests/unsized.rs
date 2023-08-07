use std::{borrow::Cow, rc::Rc, sync::Arc};

use ts_rs::TS;

#[test]
fn contains_str() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct S<'a> {
        b: Box<str>,
        c: Cow<'a, str>,
        r: Rc<str>,
        a: Arc<str>,
    }

    assert_eq!(
        S::decl(),
        "interface S { b: string, c: string, r: string, a: string, }"
    )
}
