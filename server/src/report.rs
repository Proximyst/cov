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

use ahash::AHashMap;
use metrics::counter;
use serde::Serialize;
use std::{num::NonZeroU32, sync::Arc};
use tracing::trace;
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
#[derive(Debug, thiserror::Error, Serialize, schemars::JsonSchema)]
#[error("the report was formatted incorrectly")]
#[non_exhaustive]
pub enum InvalidReport {
    /// The report failed to parse.
    ParseError,
    LineNumberInvalid,
    StatementsInvalid,
}

/// An aggregate report of covered files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Report {
    /// Every code region in this report.
    pub regions: Vec<Region>,
    // TODO: Model branches somehow.
}

/// A single code region in a report.
///
/// This can be a partial line, a full line, partial multiple lines, or full multiple lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Region {
    /// The file path this region belongs to.
    /// This can be either a relative path (usually from the repository root) or an absolute path.
    ///
    /// This is an [`Arc`] to reduce memory usage: one file can have thousands of regions, and as such share the same memory region.
    pub file: Arc<str>,

    /// Where the region starts. The tuple is `(line, column)`.
    pub from: (NonZeroU32, u32),

    /// Where the region ends. The tuple is `(line, column)`.
    ///
    /// If the column is `0`, the region ended on the previous line's end.
    /// This is an optimisation to not have to read the actual file to determine the line's length.
    /// As such, the line can be larger than the file is, by 1.
    ///
    /// The line is always greater than or equal to the `from` line.
    pub to: (NonZeroU32, u32),

    /// The number of statements in this region.
    /// The idea of a statement can vary between languages, e.g. individual function calls or JVM byte code instructions.
    ///
    /// This is not limited by the number of executions.
    pub statements: u32, // TODO: is NonZeroU32 a better choice?

    /// The number of executions this region had.
    ///
    /// This is not limited by the number of statements in the region.
    pub executions: u32,
}

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

impl<'a> TryFrom<golang::Report<'a>> for Report {
    type Error = InvalidReport;

    fn try_from(value: golang::Report) -> Result<Self, Self::Error> {
        let mut files = AHashMap::with_capacity(512); // will be dropped, while keeping the Arcs alive
        let mut regions = Vec::with_capacity(512);
        for region in value.regions {
            let file: &mut Arc<str> = files
                .entry(region.file_path)
                .or_insert_with(|| Arc::from(region.file_path));
            let file = file.clone();

            regions.push(Region {
                file,
                from: (
                    NonZeroU32::new(region.start_line).ok_or(InvalidReport::LineNumberInvalid)?,
                    region.start_column,
                ),
                to: (
                    NonZeroU32::new(region.end_line).ok_or(InvalidReport::LineNumberInvalid)?,
                    region.end_column,
                ),
                statements: region.statements,
                executions: region.executed,
            });
        }

        regions.shrink_to_fit();
        Ok(Report { regions })
    }
}

impl TryFrom<jacoco::Report> for Report {
    type Error = InvalidReport;

    fn try_from(value: jacoco::Report) -> Result<Self, Self::Error> {
        let mut files = AHashMap::with_capacity(512); // will be dropped, while keeping the Arcs alive
        let mut regions = Vec::with_capacity(512);
        for pkg in value.packages {
            for source_file in pkg.source_files {
                let file: &mut Arc<str> = files
                    .entry(source_file.name.clone())
                    .or_insert_with(|| Arc::from(source_file.name));
                let file = file.clone();

                for line in source_file.lines {
                    let nr =
                        NonZeroU32::new(line.number).ok_or(InvalidReport::LineNumberInvalid)?;
                    regions.push(Region {
                        file: file.clone(),
                        from: (nr, 0),
                        to: (
                            nr.checked_add(1).ok_or(InvalidReport::LineNumberInvalid)?,
                            0,
                        ),
                        statements: line
                            .hit_calls
                            .checked_add(line.missed_calls)
                            .ok_or(InvalidReport::StatementsInvalid)?,
                        executions: line.hit_calls,
                    });
                }
            }
        }

        regions.shrink_to_fit();
        Ok(Report { regions })
    }
}

impl<'a> TryFrom<lcov::Report<'a>> for Report {
    type Error = InvalidReport;

    fn try_from(value: lcov::Report<'a>) -> Result<Self, Self::Error> {
        let mut files = AHashMap::with_capacity(512); // will be dropped, while keeping the Arcs alive
        let mut regions = Vec::with_capacity(512);
        for record in value.record {
            let file: &mut Arc<str> = files
                .entry(record.source_file_name)
                .or_insert_with(|| Arc::from(record.source_file_name));
            let file = file.clone();

            for line in record.lines {
                let nr =
                    NonZeroU32::new(line.line_number).ok_or(InvalidReport::LineNumberInvalid)?;
                regions.push(Region {
                    file: file.clone(),
                    from: (nr, 0),
                    to: (
                        nr.checked_add(1).ok_or(InvalidReport::LineNumberInvalid)?,
                        0,
                    ),
                    // Lcov doesn't have a concept of statements per line, so we're using this as a boolean flag.
                    statements: 1,
                    executions: (line.execution_count > 0).then_some(1).unwrap_or(0),
                });
            }
        }

        regions.shrink_to_fit();
        Ok(Report { regions })
    }
}

/// Parse a coverage report from the body.
pub fn parse_report(txt: &str) -> Result<Report, InvalidReport> {
    fn parse<'a, F, I, E>(
        parser: F,
        body: &'a str,
        name: &'static str,
    ) -> Result<Option<Report>, InvalidReport>
    where
        F: FnOnce(&'a str) -> Result<I, E>,
        I: 'a + TryInto<Report, Error = InvalidReport> + std::fmt::Debug,
        E: std::fmt::Debug,
    {
        match parser(body) {
            Ok(r) => {
                counter!("cov.report.parse_report.parser_tested",
                    "outcome" => "success",
                    "parser" => name)
                .increment(1);
                trace!(report = ?r, "parsed as {}", name);

                match r.try_into() {
                    Ok(r) => {
                        counter!("cov.report.parse_report",
                            "outcome" => "success",
                            "parser" => name)
                        .increment(1);
                        trace!(report = ?r, "converted to generic report model");
                        Ok(Some(r))
                    }
                    Err(err) => {
                        counter!("cov.report.parse_report",
                            "outcome" => "conversion_failure",
                            "parser" => name)
                        .increment(1);
                        trace!(?err, "failed to convert to generic report model");
                        Err(err)
                    }
                }
            }
            Err(err) => {
                counter!("cov.report.parse_report.parser_tested",
                    "outcome" => "failure",
                    "parser" => name)
                .increment(1);
                trace!(?err, "failed to parse as {}", name);
                Ok(None)
            }
        }
    }

    if let Some(r) = parse(golang::Report::from_str, txt, "golang")? {
        return Ok(r);
    }

    if let Some(r) = parse(jacoco::Report::from_str, txt, "jacoco")? {
        return Ok(r);
    }

    if let Some(r) = parse(lcov::Report::from_str, txt, "lcov")? {
        return Ok(r);
    }

    // No parser could parse the report.
    Err(InvalidReport::ParseError)
}
