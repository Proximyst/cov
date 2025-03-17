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

mod golang;
mod jacoco;
mod lcov;

/// The input report was invalid.
/// There are no details as this should be regarded as an unrecoverable error; a new upload would be required.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("the report was formatted incorrectly")]
pub struct InvalidReport;
