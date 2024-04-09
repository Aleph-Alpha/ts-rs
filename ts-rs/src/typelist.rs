//! A simple zero-sized collection of types.

use std::{any::TypeId, marker::PhantomData};

use crate::TS;

/// A visitor used to iterate over a [`TypeList`].  
///
/// Example:
/// ```
/// # use ts_rs::TS;
/// # use ts_rs::typelist::{TypeList, TypeVisitor};
/// struct Visit;
/// impl TypeVisitor for Visit {
///   fn visit<T: TS + 'static + ?Sized>(&mut self) {
///       println!("{}", T::name());
///   }
/// }
///
/// # fn primitives() -> impl TypeList {
/// #     let signed = ().push::<i8>().push::<i16>().push::<i32>().push::<i64>();
/// #     let unsigned = ().push::<u8>().push::<u16>().push::<u32>().push::<u64>();
/// #     ().push::<char>()
/// #       .push::<bool>()
/// #       .extend(signed)
/// #       .extend(unsigned)
/// # }
/// // fn primitives() -> impl TypeList { ... }
/// primitives().for_each(&mut Visit);
/// ```
pub trait TypeVisitor: Sized {
    fn visit<T: TS + 'static + ?Sized>(&mut self);
}

/// A list containing types implementing `TS + 'static + ?Sized`.  
///
/// To construct a [`TypeList`], start with the empty list, which is the unit type `()`, and
/// repeatedly call [`TypeList::push`] or [`TypeList::extend`] on it.  
///
/// Example:
/// ```
/// # use ts_rs::typelist::TypeList;
/// fn primitives() -> impl TypeList {
///     let signed = ().push::<i8>().push::<i16>().push::<i32>().push::<i64>();
///     let unsigned = ().push::<u8>().push::<u16>().push::<u32>().push::<u64>();
///     ().push::<char>()
///       .push::<bool>()
///       .extend(signed)
///       .extend(unsigned)
/// }
/// ```
///
/// The only way to get access to the types contained in a [`TypeList`] is to iterate over it by
/// creating a visitor implementing [`TypeVisitor`] and calling [`TypeList::for_each`].
///
/// Under the hood, [`TypeList`] is recursively defined as follows:
/// - The unit type `()` is the empty [`TypeList`]
/// - For every `T: TS`, `(PhantomData<T>,)` is a [`TypeList`]
/// - For every two [`TypeList`]s `A` and `B`, `(A, B)` is a [`TypeList`]
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
