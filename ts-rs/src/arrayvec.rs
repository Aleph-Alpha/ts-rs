impl<T: crate::TS, const N: usize> crate::TS for arrayvec::ArrayVec<T, N> {
    type WithoutGenerics = arrayvec::ArrayVec<crate::Dummy, N>;
    type OptionInnerType = Self;

    fn ident() -> String {
        "Array".to_owned()
    }
    fn name() -> String {
        format!("Array<{}>", <T as crate::TS>::name())
    }
    fn inline() -> String {
        format!("Array<{}>", <T as crate::TS>::inline())
    }
    fn visit_dependencies(v: &mut impl crate::TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_dependencies(v);
    }
    fn visit_generics(v: &mut impl crate::TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_generics(v);
        v.visit::<T>();
    }
    fn decl() -> String {
        panic!("{} cannot be declared", <Self as crate::TS>::name())
    }
    fn decl_concrete() -> String {
        panic!("{} cannot be declared", <Self as crate::TS>::name())
    }
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", <Self as crate::TS>::name())
    }
}

impl<const N: usize> crate::TS for arrayvec::ArrayString<N> {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn ident() -> String {
        "string".to_owned()
    }
    fn name() -> String {
        "string".to_owned()
    }
    fn inline() -> String {
        "string".to_owned()
    }
    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", <Self as crate::TS>::name())
    }
    fn decl() -> String {
        panic!("{} cannot be declared", <Self as crate::TS>::name())
    }
    fn decl_concrete() -> String {
        panic!("{} cannot be declared", <Self as crate::TS>::name())
    }
}
