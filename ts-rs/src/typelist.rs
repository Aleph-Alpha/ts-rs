use std::{any::TypeId, marker::PhantomData};

use crate::TS;

pub trait TypeVisitor: Sized {
    fn visit<T: TS + 'static + ?Sized>(&mut self);
}

pub trait TypeList: Copy + Clone {
    fn push<T: TS + 'static + ?Sized>(self) -> impl TypeList {
        (self, (PhantomData::<T>,))
    }
    fn extend(self, l: impl TypeList) -> impl TypeList {
        (self, l)
    }

    fn contains<C: Sized + 'static>(self) -> bool;
    fn for_each(self, v: &mut impl TypeVisitor);
}

impl TypeList for () {
    fn contains<C: Sized>(self) -> bool {
        false
    }
    fn for_each(self, _: &mut impl TypeVisitor) {}
}

impl<T> TypeList for (PhantomData<T>,)
where
    T: TS + 'static + ?Sized,
{
    fn contains<C: Sized + 'static>(self) -> bool {
        TypeId::of::<C>() == TypeId::of::<T>()
    }

    fn for_each(self, v: &mut impl TypeVisitor) {
        v.visit::<T>();
    }
}

impl<A, B> TypeList for (A, B)
where
    A: TypeList,
    B: TypeList,
{
    fn contains<C: Sized + 'static>(self) -> bool {
        self.0.contains::<C>() || self.1.contains::<C>()
    }

    fn for_each(self, v: &mut impl TypeVisitor) {
        self.0.for_each(v);
        self.1.for_each(v);
    }
}
