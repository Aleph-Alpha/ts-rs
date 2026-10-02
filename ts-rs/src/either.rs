use crate::{impl_shadow, TS};

#[derive(TS)]
#[ts(crate = "crate", rename = "Either", export_to = "either/")]
pub enum ToEither<L, R> {
    /// A value of type `L`.
    Left(L),
    /// A value of type `R`.
    Right(R),
}
impl_shadow!(as ToEither<L, R>: impl <L: TS, R: TS> TS for ::either::Either<L, R>);
