use crate::{impl_shadow, TS};

impl_shadow!(as Vec<T>: impl <T: TS> TS for ::nonempty::NonEmpty<T>);
