use super::InvalidReport;
use std::str::FromStr;
use winnow::{
    Parser, Result,
    ascii::{dec_uint, line_ending, newline},
    combinator::{alt, opt, separated, terminated},
    token::{literal, take_until},
};

/// A Go coverage report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// The mode of the counting in the Go report.
    pub mode: Mode,
    /// The code regions of the Go report.
    pub regions: Vec<LineRegion>,
}

impl FromStr for Report {
    type Err = InvalidReport;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let s = &mut s;
        let mode = parse_mode(s).map_err(|_| InvalidReport)?;
        let regions = separated(0.., parse_region, line_ending)
            .parse_next(s)
            .map_err(|_| InvalidReport)?;
        if s.is_empty() {
            Ok(Report { mode, regions })
        } else {
            Err(InvalidReport)
        }
    }
}

/// The mode of the counting in the Go report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Did the statement run at all?
    Set,
    /// How many times did the statement run?
    /// Unsafe in concurrent settings.
    Count,
    /// How many times did the statement run?
    /// Safe in concurrent settings.
    Atomic,
}

fn parse_mode(s: &mut &str) -> Result<Mode> {
    let _ = literal("mode: ").parse_next(s)?;
    let mode = alt((
        literal("set").map(|_| Mode::Set),
        literal("count").map(|_| Mode::Count),
        literal("atomic").map(|_| Mode::Atomic),
    ))
    .parse_next(s)?;
    let _ = opt(newline).parse_next(s)?;
    Ok(mode)
}

/// A line region in the Go report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineRegion {
    /// The file path this region belongs to.
    /// This is usually a relative path from the root of the project.
    pub file_path: String,
    /// The starting line of the region.
    pub start_line: u32,
    /// The ending line of the region.
    pub end_line: u32,
    /// The starting column of the region.
    pub start_column: u32,
    /// The ending column of the region.
    pub end_column: u32,
    /// The number of statements in this region.
    pub statements: u32,
    /// The number of times this region was executed.
    /// This is not limited by [statements].
    pub executed: u32,
}

fn parse_region(s: &mut &str) -> Result<LineRegion> {
    // format: "file_path:start_line.start_column,end_line.end_column statements executed"
    let file_path = terminated(take_until(1.., ":"), ":").parse_next(s)?.into();
    let start_line = terminated(dec_uint, ".").parse_next(s)?;
    let start_column = terminated(dec_uint, ",").parse_next(s)?;
    let end_line = terminated(dec_uint, ".").parse_next(s)?;
    let end_column = terminated(dec_uint, " ").parse_next(s)?;
    let statements = terminated(dec_uint, " ").parse_next(s)?;
    let executed = dec_uint.parse_next(s)?;

    Ok(LineRegion {
        file_path,
        start_line,
        start_column,
        end_line,
        end_column,
        statements,
        executed,
    })
}

#[cfg(test)]
mod tests {
    use super::{InvalidReport, LineRegion, Mode, Report};
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn valid_set_report() {
        let report = r#"mode: set
github.com/owner/repo/file.go:1.2,3.4 5 6"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                mode: Mode::Set,
                regions: vec![LineRegion {
                    file_path: "github.com/owner/repo/file.go".into(),
                    start_line: 1,
                    start_column: 2,
                    end_line: 3,
                    end_column: 4,
                    statements: 5,
                    executed: 6,
                }],
            },
        );
    }

    #[test]
    fn valid_count_report() {
        let report = r#"mode: count
github.com/owner/repo/file.go:1.2,3.4 5 6"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                mode: Mode::Count,
                regions: vec![LineRegion {
                    file_path: "github.com/owner/repo/file.go".into(),
                    start_line: 1,
                    start_column: 2,
                    end_line: 3,
                    end_column: 4,
                    statements: 5,
                    executed: 6,
                }],
            },
        );
    }

    #[test]
    fn valid_atomic_report() {
        let report = r#"mode: atomic
github.com/owner/repo/file.go:1.2,3.4 5 6"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                mode: Mode::Atomic,
                regions: vec![LineRegion {
                    file_path: "github.com/owner/repo/file.go".into(),
                    start_line: 1,
                    start_column: 2,
                    end_line: 3,
                    end_column: 4,
                    statements: 5,
                    executed: 6,
                }],
            },
        );
    }

    #[test]
    fn reads_zero_regions() {
        let report = "mode: atomic";
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                mode: Mode::Atomic,
                regions: Vec::new(),
            },
        );
    }

    #[test]
    fn reads_multiple_regions() {
        let report = "mode: atomic
github.com/owner/repo/file.go:1.2,3.4 5 6
github.com/owner/repo/file.go:7.8,9.10 11 12
github.com/owner/repo/file.go:13.14,15.16 17 18";
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                mode: Mode::Atomic,
                regions: vec![
                    LineRegion {
                        file_path: "github.com/owner/repo/file.go".into(),
                        start_line: 1,
                        start_column: 2,
                        end_line: 3,
                        end_column: 4,
                        statements: 5,
                        executed: 6,
                    },
                    LineRegion {
                        file_path: "github.com/owner/repo/file.go".into(),
                        start_line: 7,
                        start_column: 8,
                        end_line: 9,
                        end_column: 10,
                        statements: 11,
                        executed: 12,
                    },
                    LineRegion {
                        file_path: "github.com/owner/repo/file.go".into(),
                        start_line: 13,
                        start_column: 14,
                        end_line: 15,
                        end_column: 16,
                        statements: 17,
                        executed: 18,
                    },
                ],
            },
        );
    }

    #[test]
    fn invalid_format_returns_err() {
        let report = "mode: unknown
github.com/owner/repo/file.go:1.2,3.4 5 6";
        let report = Report::from_str(report);
        assert_eq!(report, Err(InvalidReport));

        let report = "mode: atomic
github.com/owner/repo/file.go:1,2.3,4 5 6";
        let report = Report::from_str(report);
        assert_eq!(report, Err(InvalidReport));

        let report = "mode: atomic
github.com/owner/repo/file.go";
        let report = Report::from_str(report);
        assert_eq!(report, Err(InvalidReport));

        let report = "mode: atomic
:1.2,3.4 5 6";
        let report = Report::from_str(report);
        assert_eq!(report, Err(InvalidReport));
    }
}
