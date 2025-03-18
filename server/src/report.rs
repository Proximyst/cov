//! Parsing and modelling of language-specific coverage reports.
//!
//! We support:
//!   * Go: Go has its own coverage format, and does not easily support any other.
//!   * JaCoCo: Java, Kotlin, Groovy, Scala, and more support this.
//!   * Lcov: Rust, C, C++, Jest, and more support this.
//!
//! We're ignoring:
//!   * clover: Java has JaCoCo. Jest has Lcov.
//!   * cobertura: CXX have Lcov. Jest has Lcov.
//!   * Jest JSON: Jest has Lcov.

use winnow::{
    ModalParser, Parser,
    error::{AddContext, ErrMode, ParserError, StrContext},
    stream::Stream,
};

mod golang;
mod jacoco;
mod lcov;

/// The input report was invalid.
/// There are no details as this should be regarded as an unrecoverable error; a new upload would be required.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("the report was formatted incorrectly")]
pub struct InvalidReport;

/// Handy extensions for parsers.
trait ParserExt<I, O, E> {
    fn ctx(self, context: &'static str) -> impl ModalParser<I, O, E>;
}

impl<P, I, O, E> ParserExt<I, O, E> for P
where
    P: ModalParser<I, O, E>,
    P: Parser<I, O, ErrMode<E>>,
    E: ParserError<I> + AddContext<I, StrContext>,
    I: Stream,
{
    fn ctx(self, context: &'static str) -> impl ModalParser<I, O, E> {
        self.context(StrContext::Label(context))
    }
}
