use aide::OperationOutput;
use axum::response::{IntoResponse, Response};
use either::Either;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

/// A type that can be either `L` or `R`. Analogous to [`Either`], but with Axum support.
pub enum AxumEither<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Debug for AxumEither<L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxumEither::Left(l) => f.debug_tuple("AxumEither::Left").field(l).finish(),
            AxumEither::Right(r) => f.debug_tuple("AxumEither::Right").field(r).finish(),
        }
    }
}

impl<L, R> IntoResponse for AxumEither<L, R>
where
    L: IntoResponse,
    R: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            AxumEither::Left(l) => l.into_response(),
            AxumEither::Right(r) => r.into_response(),
        }
    }
}

impl<L, R> OperationOutput for AxumEither<L, R>
where
    L: OperationOutput,
    R: OperationOutput,
{
    type Inner = Self;
}

impl<L, R> From<Either<L, R>> for AxumEither<L, R> {
    fn from(either: Either<L, R>) -> Self {
        match either {
            Either::Left(l) => AxumEither::Left(l),
            Either::Right(r) => AxumEither::Right(r),
        }
    }
}

impl<L, R> Into<Either<L, R>> for AxumEither<L, R> {
    fn into(self) -> Either<L, R> {
        match self {
            AxumEither::Left(l) => Either::Left(l),
            AxumEither::Right(r) => Either::Right(r),
        }
    }
}

impl<L, R> Serialize for AxumEither<L, R>
where
    L: Serialize,
    R: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AxumEither::Left(l) => l.serialize(serializer),
            AxumEither::Right(r) => r.serialize(serializer),
        }
    }
}

impl<'de, L, R> Deserialize<'de> for AxumEither<L, R>
where
    L: Deserialize<'de>,
    R: Deserialize<'de>,
    Self: Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let either = Either::<L, R>::deserialize(deserializer)?;
        Ok(AxumEither::from(either))
    }
}
