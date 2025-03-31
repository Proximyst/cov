//! This format is described in the [Lcov documentation](https://github.com/linux-test-project/lcov/blob/38a82d5d03c449b4253223c111aef1c36e46d5db/man/geninfo.1#L1370).
//! It can also be accessed via `man geninfo -1` on a system with `lcov` installed; find the `TRACEFILE FORMAT` section.

use super::ParserExt;
use std::collections::BTreeMap;
use tracing::trace;
use winnow::{
    ModalResult, Parser, Result,
    ascii::{dec_uint, line_ending, till_line_ending},
    combinator::{alt, cut_err, fail, opt, preceded, terminated},
    error::{ContextError, ParseError},
    token::take_until,
};

/// A report encapsulates a file of coverage details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report<'a> {
    /// Each indiviudal record this file is composed of.
    pub record: Vec<Record<'a>>,
}

/// One individual test and its coverage.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Record<'a> {
    /// The name of the test. Can be an empty string.
    pub test_name: &'a str,

    /// The name of the source file.
    /// Can be relative or absolute. If relative, it is assumed to be relative to the root of the repository.
    pub source_file_name: &'a str,

    /// The functions in the source file for this test.
    pub functions: Vec<Function<'a>>,

    /// How many functions were found in this source file.
    ///
    /// TODO: What is a "function group"?
    pub functions_found: u32,

    /// How many functions were hit in this source file.
    /// This is always less than or equal to `functions_found`.
    pub functions_hit: u32,

    /// The branches in the source file for this test.
    pub branches: Vec<Branch<'a>>,

    /// How many branches were found in this source file.
    pub branches_found: u32,

    /// How many branches were hit in this source file.
    /// This is always less than or equal to `branches_found`.
    pub branches_hit: u32,

    /// The MCDC coverage for this test.
    pub mcdc: Vec<Mcdc<'a>>,

    /// How many modified coverage conditions were found in this source file.
    pub modified_coverage_conditions_found: u32,

    /// How many modified coverage conditions were hit in this source file.
    /// This is always less than or equal to `modified_coverage_conditions_found`.
    pub modified_coverage_conditions_hit: u32,

    /// The lines in the source file for this test.
    pub lines: Vec<CoveredLine<'a>>,

    /// How many lines were hit in this source file.
    pub lines_hit: u32,

    /// How many lines were found in this source file.
    /// This is always less than or equal to `lines_hit`.
    pub lines_found: u32,
}

/// A function in the source code. May be linked to multiple tests for the same source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<'a> {
    /// The name of the function. May be mangled, whose format depends on the source file type.
    /// Technically, a function can have multiple names, but this doesn't particularly help; we choose the first one to represent the function and sum the alias executions.
    pub name: &'a str,
    /// The line number where the function starts.
    pub line_number_start: u32,
    /// The line number where the function ends, if this is reported.
    /// `line_number_end` is always greater than or equal to `line_number_start`.
    pub line_number_end: Option<u32>,
    /// How many times this function was executed.
    pub execution_count: u32,
    /// The full list of aliases this function has, along with their individual execution counts.
    pub aliases: Vec<(&'a str, u32)>,
}

/// A branch in the source code. May be linked to multiple tests for the same source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch<'a> {
    /// The line number where the branch is located.
    pub line_number: u32,
    /// Whether this branch is an exception branch.
    /// Not all coverage tools support this, so this may be incorrect if `false`.
    pub exception: bool,
    /// The block number of this branch.
    pub block: u32,
    /// An identifier for this branch. This is tool-specific, and may be any arbitrary string (including commas).
    pub branch: &'a str,
    /// How many times this branch was taken.
    pub taken: u32,
}

/// MD/DC: Modified Decision/Condition Coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mcdc<'a> {
    /// The line number where this MCDC group is located.
    pub line_number: u32,
    /// The number of conditions in this MCDC group.
    pub group_size: u32,
    /// Whether this MCDC group is inverted.
    pub sense: bool,
    /// Some tools treat this as a boolean, others as a counter.
    pub taken: u32,
    /// The index is at least `0`, and at most `group_size - 1`.
    pub index: u32,
    /// The expression for this MCDC group.
    /// This is useful to humans and is tool-specific.
    pub expression: &'a str,
}

/// Per-line coverage data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoveredLine<'a> {
    /// The line number where this coverage data is located.
    pub line_number: u32,
    /// How many times this line was executed.
    pub execution_count: u32,
    /// An optional checksum for this line. This is tool-specific.
    pub checksum: Option<&'a str>,
}

impl<'a> Report<'a> {
    #[allow(dead_code)] // TODO: Remove this.
    // TODO(perf): don't use ContextError outside tests
    pub fn from_str(s: &'a str) -> Result<Self, ParseError<&'a str, ContextError>> {
        parse_report.ctx("parsing report").parse(s.trim())
    }
}

/// Parses a report.
///
/// This acts as a state machine: it will read each line, slowly building up a [`Report`] to return.
/// When all lines are consumed, it will return the report.
fn parse_report<'a>(s: &mut &'a str) -> ModalResult<Report<'a>> {
    while opt(terminated(parse_comment.ctx("comment"), opt(line_ending)))
        .ctx("discarding all comments")
        .parse_next(s)?
        .is_some()
    {}

    let mut tests = Vec::new();

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum FnKey<'a> {
        Modern(u32),
        Legacy(&'a str),
    }
    let mut record = Record::default();
    let mut functions = BTreeMap::new();

    while !s.is_empty() {
        let line = cut_err(parse_input_line)
            .ctx("parsing input line")
            .parse_next(s)?;
        opt(line_ending).parse_next(s)?;

        match line {
            InputLine::Comment(comment) => trace!(comment, "skipping comment"),
            InputLine::TestName(name) => {
                if !record.test_name.is_empty() {
                    fail.ctx("two test name fields were given").parse_next(s)?;
                }
                record.test_name = name;
            }
            InputLine::SourceFileName(name) => {
                if !record.source_file_name.is_empty() {
                    fail.ctx("two source file name fields were given")
                        .parse_next(s)?;
                }
                record.source_file_name = name;
            }
            InputLine::SourceCodeVersion(version) => {
                trace!(version, "skipping source code version")
            }
            InputLine::BranchesHit(hit) => record.branches_hit = hit,
            InputLine::BranchesFound(found) => record.branches_found = found,
            InputLine::FunctionsHit(hit) => record.functions_hit = hit,
            InputLine::FunctionsFound(found) => record.functions_found = found,
            InputLine::LinesHit(hit) => record.lines_hit = hit,
            InputLine::LinesFound(found) => record.lines_found = found,
            InputLine::McdcHit(hit) => record.modified_coverage_conditions_hit = hit,
            InputLine::McdcFound(found) => record.modified_coverage_conditions_found = found,
            InputLine::LineData(da) => {
                record.lines.push(CoveredLine {
                    line_number: da.line_number,
                    execution_count: da.execution_count,
                    checksum: da.checksum,
                });
            }
            InputLine::Mcdc(mcdc) => {
                record.mcdc.push(Mcdc {
                    line_number: mcdc.line_number,
                    group_size: mcdc.group_size,
                    sense: mcdc.sense,
                    taken: mcdc.taken,
                    index: mcdc.index,
                    expression: mcdc.expression,
                });
            }
            InputLine::Branch(brda) => {
                record.branches.push(Branch {
                    line_number: brda.line_number,
                    exception: brda.exception,
                    block: brda.block,
                    branch: brda.branch,
                    taken: brda.taken,
                });
            }
            InputLine::ModernFunctionLeader(leader) => {
                functions.insert(
                    FnKey::Modern(leader.index),
                    Function {
                        name: "",
                        line_number_start: leader.line_number,
                        line_number_end: leader.line_number_end,
                        execution_count: 0,
                        aliases: Vec::new(),
                    },
                );
            }
            InputLine::ModernFunctionAlias(alias) => {
                let Some(f) = functions.get_mut(&FnKey::Modern(alias.index)) else {
                    fail.ctx("function alias without leader").parse_next(s)?
                };

                if f.name.is_empty() {
                    f.name = alias.name;
                }
                f.execution_count += alias.execution_count;
                f.aliases.push((alias.name, alias.execution_count));
            }
            InputLine::LegacyFunctionLeader(leader) => {
                functions.insert(
                    FnKey::Legacy(leader.name),
                    Function {
                        name: leader.name,
                        line_number_start: leader.line_number,
                        line_number_end: leader.line_number_end,
                        execution_count: 0,
                        aliases: Vec::new(),
                    },
                );
            }
            InputLine::LegacyFunctionData(data) => {
                let Some(f) = functions.get_mut(&FnKey::Legacy(data.name)) else {
                    fail.ctx("function data without leader").parse_next(s)?
                };

                f.execution_count += data.execution_count;
                f.aliases.push((f.name, data.execution_count));
            }
            InputLine::EndOfRecord => {
                for f in functions.values() {
                    if f.name.is_empty() {
                        fail.ctx("an input function has no name").parse_next(s)?;
                    }
                }

                record.functions = functions.values().cloned().collect::<Vec<_>>();
                tests.push(std::mem::take(&mut record));

                // We clear instead of take to avoid reallocating a potentially big map.
                functions.clear();
            }
        }
    }
    if record != Record::default() {
        // No end_of_record was listed.
        fail.ctx("no end_of_record was listed").parse_next(s)?;
    }

    Ok(Report { record: tests })
}

#[derive(Debug, PartialEq, Eq)]
enum InputLine<'a> {
    Comment(&'a str),
    TestName(&'a str),
    SourceFileName(&'a str),
    SourceCodeVersion(&'a str),
    LinesHit(u32),
    LinesFound(u32),
    McdcFound(u32),
    McdcHit(u32),
    BranchesFound(u32),
    BranchesHit(u32),
    FunctionsFound(u32),
    FunctionsHit(u32),
    LineData(DA<'a>),
    Mcdc(RawMCDC<'a>),
    Branch(Brda<'a>),
    ModernFunctionLeader(ModernFunctionLeader),
    ModernFunctionAlias(ModernFunctionAlias<'a>),
    LegacyFunctionLeader(LegacyFunctionLeader<'a>),
    LegacyFunctionData(LegacyFunctionData<'a>),
    EndOfRecord,
}

/// Parses one line of input. Does not consume the line ending.
fn parse_input_line<'s>(s: &mut &'s str) -> ModalResult<InputLine<'s>> {
    alt((
        parse_comment.map(InputLine::Comment).ctx("comment"),
        parse_tn.map(InputLine::TestName).ctx("test name"),
        parse_sf
            .map(InputLine::SourceFileName)
            .ctx("source file name"),
        parse_ver
            .map(InputLine::SourceCodeVersion)
            .ctx("source code version"),
        parse_lf.map(InputLine::LinesFound).ctx("lines found"),
        parse_lh.map(InputLine::LinesHit).ctx("lines hit"),
        parse_mrh.map(InputLine::McdcHit).ctx("mcdc hit"),
        parse_mrf.map(InputLine::McdcFound).ctx("mcdc found"),
        parse_brf
            .map(InputLine::BranchesFound)
            .ctx("branches found"),
        parse_brh.map(InputLine::BranchesHit).ctx("branches hit"),
        parse_fnf
            .map(InputLine::FunctionsFound)
            .ctx("functions found"),
        parse_fnh.map(InputLine::FunctionsHit).ctx("functions hit"),
        parse_da.map(InputLine::LineData).ctx("line data"),
        parse_mcdc.map(InputLine::Mcdc).ctx("mcdc"),
        parse_brda.map(InputLine::Branch).ctx("branch"),
        parse_fnl
            .map(InputLine::ModernFunctionLeader)
            .ctx("modern function leader"),
        parse_fna
            .map(InputLine::ModernFunctionAlias)
            .ctx("modern function alias"),
        parse_fn
            .map(InputLine::LegacyFunctionLeader)
            .ctx("legacy function leader"),
        parse_fnda
            .map(InputLine::LegacyFunctionData)
            .ctx("legacy function data"),
        "end_of_record"
            .map(|_| InputLine::EndOfRecord)
            .ctx("end of record"),
        fail.ctx("unknown field type"),
    ))
    .parse_next(s)
}

fn parse_comment<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
    "#".parse_next(s)?;
    till_line_ending.parse_next(s)
}

fn parse_tn<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
    "TN:".parse_next(s)?;
    till_line_ending.parse_next(s)
}

fn parse_sf<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
    "SF:".parse_next(s)?;
    till_line_ending.parse_next(s)
}

fn parse_ver<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
    "VER:".parse_next(s)?;
    till_line_ending.parse_next(s)
}

fn parse_lh(s: &mut &str) -> ModalResult<u32> {
    "LH:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_lf(s: &mut &str) -> ModalResult<u32> {
    "LF:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_mrf(s: &mut &str) -> ModalResult<u32> {
    "MRF:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_mrh(s: &mut &str) -> ModalResult<u32> {
    "MRH:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_brf(s: &mut &str) -> ModalResult<u32> {
    "BRF:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_brh(s: &mut &str) -> ModalResult<u32> {
    "BRH:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_fnf(s: &mut &str) -> ModalResult<u32> {
    "FNF:".parse_next(s)?;
    dec_uint.parse_next(s)
}

fn parse_fnh(s: &mut &str) -> ModalResult<u32> {
    "FNH:".parse_next(s)?;
    dec_uint.parse_next(s)
}

#[derive(Debug, PartialEq, Eq)]
struct DA<'a> {
    line_number: u32,
    execution_count: u32,
    checksum: Option<&'a str>,
}

fn parse_da<'s>(s: &mut &'s str) -> ModalResult<DA<'s>> {
    "DA:".parse_next(s)?;
    let line_number = terminated(dec_uint, ",").ctx("line_number").parse_next(s)?;
    let execution_count = dec_uint.ctx("execution_count").parse_next(s)?;
    let checksum = opt(preceded(",", till_line_ending))
        .ctx("checksum")
        .parse_next(s)?;
    Ok(DA {
        line_number,
        execution_count,
        checksum,
    })
}

#[derive(Debug, PartialEq, Eq)]
struct RawMCDC<'a> {
    line_number: u32,
    group_size: u32,
    sense: bool,
    taken: u32,
    index: u32,
    expression: &'a str,
}

fn parse_mcdc<'s>(s: &mut &'s str) -> ModalResult<RawMCDC<'s>> {
    "MCDC:".parse_next(s)?;
    let line_number = terminated(dec_uint, ",").ctx("line_number").parse_next(s)?;
    let group_size = terminated(dec_uint, ",").ctx("group_size").parse_next(s)?;
    let sense = terminated(alt(("t", "f")), ",")
        .map(|s| s == "t")
        .ctx("sense")
        .parse_next(s)?;
    let taken = terminated(dec_uint, ",").ctx("taken").parse_next(s)?;
    let index = terminated(dec_uint, ",").ctx("index").parse_next(s)?;
    let expression = till_line_ending.parse_next(s)?;
    Ok(RawMCDC {
        line_number,
        group_size,
        sense,
        taken,
        index,
        expression,
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Brda<'a> {
    line_number: u32,
    exception: bool,
    block: u32,
    branch: &'a str,
    taken: u32,
}

fn parse_brda<'s>(s: &mut &'s str) -> ModalResult<Brda<'s>> {
    "BRDA:".parse_next(s)?;
    let line_number = terminated(dec_uint, ",").ctx("line_number").parse_next(s)?;
    let exception = opt("e")
        .map(|o| o.is_some())
        .ctx("exception")
        .parse_next(s)?;
    let block = terminated(dec_uint, ",").ctx("block").parse_next(s)?;
    let branch = take_until(0.., ',').ctx("branch").parse_next(s)?;
    ','.parse_next(s)?;
    let taken = alt((dec_uint, '-'.map(|_| 0))).ctx("taken").parse_next(s)?;

    Ok(Brda {
        line_number,
        exception,
        block,
        branch,
        taken,
    })
}

#[derive(Debug, PartialEq, Eq)]
struct ModernFunctionLeader {
    pub index: u32,
    pub line_number: u32,
    pub line_number_end: Option<u32>,
}

fn parse_fnl(s: &mut &str) -> ModalResult<ModernFunctionLeader> {
    "FNL:".parse_next(s)?;
    let index = terminated(dec_uint, ",").ctx("index").parse_next(s)?;
    let line_number = dec_uint.ctx("line_number").parse_next(s)?;
    let line_number_end = opt(preceded(",", dec_uint))
        .ctx("line_number_end")
        .parse_next(s)?;

    Ok(ModernFunctionLeader {
        index,
        line_number,
        line_number_end,
    })
}

#[derive(Debug, PartialEq, Eq)]
struct ModernFunctionAlias<'a> {
    pub index: u32,
    pub execution_count: u32,
    pub name: &'a str,
}

fn parse_fna<'s>(s: &mut &'s str) -> ModalResult<ModernFunctionAlias<'s>> {
    "FNA:".parse_next(s)?;
    let index = terminated(dec_uint, ",").ctx("index").parse_next(s)?;
    let execution_count = terminated(dec_uint, ",")
        .ctx("execution_count")
        .parse_next(s)?;
    let name = till_line_ending.parse_next(s)?;
    Ok(ModernFunctionAlias {
        index,
        execution_count,
        name,
    })
}

#[derive(Debug, PartialEq, Eq)]
struct LegacyFunctionLeader<'a> {
    pub line_number: u32,
    pub line_number_end: Option<u32>,
    pub name: &'a str,
}

fn parse_fn<'s>(s: &mut &'s str) -> ModalResult<LegacyFunctionLeader<'s>> {
    "FN:".parse_next(s)?;
    let line_number = terminated(dec_uint, ",").ctx("line_number").parse_next(s)?;
    let line_number_end = opt(terminated(dec_uint, ","))
        .ctx("line_number_end")
        .parse_next(s)?;
    let name = till_line_ending.ctx("name").parse_next(s)?;
    Ok(LegacyFunctionLeader {
        line_number,
        line_number_end,
        name,
    })
}

#[derive(Debug, PartialEq, Eq)]
struct LegacyFunctionData<'a> {
    pub execution_count: u32,
    pub name: &'a str,
}

fn parse_fnda<'s>(s: &mut &'s str) -> ModalResult<LegacyFunctionData<'s>> {
    "FNDA:".parse_next(s)?;
    let execution_count = terminated(dec_uint, ",")
        .ctx("execution_count")
        .parse_next(s)?;
    let name = till_line_ending.ctx("name").parse_next(s)?;
    Ok(LegacyFunctionData {
        execution_count,
        name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_comment() {
        let mut input = "#this is a test comment";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::Comment("this is a test comment"))
        );
    }

    #[test]
    fn parse_test_name() {
        let mut input = "TN:";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::TestName("")));

        let mut input = "TN:test test:::";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::TestName("test test:::"))
        );
    }

    #[test]
    fn parse_source_file_name() {
        let mut input = "SF:";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::SourceFileName(""))
        );

        let mut input = "SF:test test:::";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::SourceFileName("test test:::"))
        );
    }

    #[test]
    fn parse_source_code_version() {
        let mut input = "VER:";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::SourceCodeVersion(""))
        );

        let mut input = "VER:test test:::";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::SourceCodeVersion("test test:::"))
        );
    }

    #[test]
    fn parse_lines_hit() {
        let mut input = "LH:1";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::LinesHit(1)));
    }

    #[test]
    fn parse_lines_found() {
        let mut input = "LF:1";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::LinesFound(1)));
    }

    #[test]
    fn parse_branches_found() {
        let mut input = "BRF:1";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::BranchesFound(1))
        );
    }

    #[test]
    fn parse_branches_hit() {
        let mut input = "BRH:1";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::BranchesHit(1)));
    }

    #[test]
    fn parse_functions_found() {
        let mut input = "FNF:1";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::FunctionsFound(1))
        );
    }

    #[test]
    fn parse_functions_hit() {
        let mut input = "FNH:1";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::FunctionsHit(1)));
    }

    #[test]
    fn parse_mcdc_found() {
        let mut input = "MRF:1";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::McdcFound(1)));
    }

    #[test]
    fn parse_mcdc_hit() {
        let mut input = "MRH:1";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::McdcHit(1)));
    }

    #[test]
    fn parse_end_of_record() {
        let mut input = "end_of_record";
        assert_eq!(parse_input_line(&mut input), Ok(InputLine::EndOfRecord));
    }

    #[test]
    fn parse_line_data() {
        let mut input = "DA:1,2";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::LineData(DA {
                line_number: 1,
                execution_count: 2,
                checksum: None,
            }))
        );

        let mut input = "DA:3,4,test,,";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::LineData(DA {
                line_number: 3,
                execution_count: 4,
                checksum: Some("test,,"),
            }))
        );
    }

    #[test]
    fn parse_mcdc() {
        let mut input = "MCDC:1,2,t,3,4,expression";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::Mcdc(RawMCDC {
                line_number: 1,
                group_size: 2,
                sense: true,
                taken: 3,
                index: 4,
                expression: "expression",
            }))
        );

        let mut input = "MCDC:1,2,f,3,4,,test!";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::Mcdc(RawMCDC {
                line_number: 1,
                group_size: 2,
                sense: false,
                taken: 3,
                index: 4,
                expression: ",test!",
            }))
        );
    }

    #[test]
    fn parse_branch() {
        let mut input = "BRDA:1,2,branch,3";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::Branch(Brda {
                line_number: 1,
                exception: false,
                block: 2,
                branch: "branch",
                taken: 3,
            }))
        );

        let mut input = "BRDA:1,e2,branch,-";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::Branch(Brda {
                line_number: 1,
                exception: true,
                block: 2,
                branch: "branch",
                taken: 0,
            }))
        );
    }

    #[test]
    fn parse_modern_function_leader() {
        let mut input = "FNL:1,2";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::ModernFunctionLeader(ModernFunctionLeader {
                index: 1,
                line_number: 2,
                line_number_end: None,
            }))
        );

        let mut input = "FNL:1,2,3";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::ModernFunctionLeader(ModernFunctionLeader {
                index: 1,
                line_number: 2,
                line_number_end: Some(3),
            }))
        );
    }

    #[test]
    fn parse_modern_function_alias() {
        let mut input = "FNA:1,2,test!,,";
        assert_eq!(
            parse_input_line(&mut input),
            Ok(InputLine::ModernFunctionAlias(ModernFunctionAlias {
                index: 1,
                execution_count: 2,
                name: "test!,,",
            }))
        );
    }

    #[test]
    fn fails_on_lack_of_eor() {
        let report = r#"
TN:test
SF:test.js
"#;
        let res = Report::from_str(report);
        assert!(res.is_err(), "report parsed successfully: {res:#?}");

        // Make sure that the corrected version does parse.
        let report = r#"
TN:test
SF:test.js
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_ok(), "corrected fail-safe did not parse: {res:#?}");
    }

    #[test]
    fn fails_on_duplicate_test_name() {
        let report = r#"
TN:test
TN:test2
SF:test.js
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_err(), "report parsed successfully: {res:#?}");

        // Make sure that the corrected version does parse.
        let report = r#"
TN:test
SF:test.js
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_ok(), "corrected fail-safe did not parse: {res:#?}");
    }

    #[test]
    fn fails_on_duplicate_source_file_name() {
        let report = r#"
TN:test
SF:test.js
SF:test2.js
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_err(), "report parsed successfully: {res:#?}");

        // Make sure that the corrected version does parse.
        let report = r#"
TN:test
SF:test.js
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_ok(), "corrected fail-safe did not parse: {res:#?}");
    }

    #[test]
    fn fails_on_function_alias_without_leader() {
        let report = r#"
TN:test
SF:test.js
FNA:1,2,test
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_err(), "report parsed successfully: {res:#?}");

        // Make sure that the corrected version does parse.
        let report = r#"
TN:test
SF:test.js
FNL:1,2
FNA:1,2,test
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_ok(), "corrected fail-safe did not parse: {res:#?}");
    }

    #[test]
    fn fails_on_function_without_name() {
        let report = r#"
TN:test
SF:test.js
FNL:1,2
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_err(), "report parsed successfully: {res:#?}");

        // Make sure that the corrected version does parse.
        let report = r#"
TN:test
SF:test.js
FNL:1,2
FNA:1,2,test
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_ok(), "corrected fail-safe did not parse: {res:#?}");
    }

    #[test]
    fn fails_on_function_data_without_leader() {
        let report = r#"
TN:test
SF:test.js
FNDA:1,test
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_err(), "report parsed successfully: {res:#?}");

        // Make sure that the corrected version does parse.
        let report = r#"
TN:test
SF:test.js
FN:1,2,test
FNDA:1,test
end_of_record
"#;
        let res = Report::from_str(report);
        assert!(res.is_ok(), "corrected fail-safe did not parse: {res:#?}");
    }

    #[test]
    fn parses_valid_c_coverage() {
        let report = r#"
#this is a test comment
TN:
SF:/home/mariell/work/cov/samples/c/helpers.c
VER:unittest
FNL:0,3,5
FNA:0,501,min
FNL:1,7,9
FNA:1,1,max
FNF:2
FNH:2
DA:3,501
DA:4,501
DA:7,1
DA:8,1
LF:4
LH:4
end_of_record
TN:
SF:/home/mariell/work/cov/samples/c/sample.c
FNL:0,4,16
FNA:0,501,add
FNF:1
FNH:1
BRDA:5,0,0,1
BRDA:5,0,1,500
BRDA:5,0,2,1
BRDA:5,0,3,0
BRDA:10,0,0,500
BRDA:10,0,1,0
BRDA:10,0,2,0
BRDA:10,0,3,500
BRF:8
BRH:5
DA:4,501
DA:5,501
DA:7,1
DA:10,500
DA:12,0
DA:15,500
LF:6
LH:5
end_of_record
TN:
SF:/home/mariell/work/cov/samples/c/sample_test.c
FNL:0,4,20
FNA:0,1,main
FNF:1
FNH:1
BRDA:5,0,0,500
BRDA:5,0,1,1
BRDA:7,0,0,0
BRDA:7,0,1,500
BRDA:14,0,0,0
BRDA:14,0,1,1
BRF:6
BRH:4
DA:4,1
DA:5,501
DA:6,500
DA:7,500
DA:8,0
DA:9,0
DA:13,1
DA:14,1
DA:15,0
DA:16,0
DA:19,1
LF:11
LH:7
end_of_record
"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                record: vec![
                    Record {
                        test_name: "",
                        source_file_name: "/home/mariell/work/cov/samples/c/helpers.c",
                        functions: vec![
                            Function {
                                name: "min",
                                line_number_start: 3,
                                line_number_end: Some(5),
                                execution_count: 501,
                                aliases: vec![("min", 501)],
                            },
                            Function {
                                name: "max",
                                line_number_start: 7,
                                line_number_end: Some(9),
                                execution_count: 1,
                                aliases: vec![("max", 1)],
                            },
                        ],
                        functions_found: 2,
                        functions_hit: 2,
                        branches: vec![],
                        branches_found: 0,
                        branches_hit: 0,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 3,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 4,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 7,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 8,
                                execution_count: 1,
                                checksum: None,
                            },
                        ],
                        lines_hit: 4,
                        lines_found: 4,
                    },
                    Record {
                        test_name: "",
                        source_file_name: "/home/mariell/work/cov/samples/c/sample.c",
                        functions: vec![Function {
                            name: "add",
                            line_number_start: 4,
                            line_number_end: Some(16),
                            execution_count: 501,
                            aliases: vec![("add", 501)],
                        }],
                        functions_found: 1,
                        functions_hit: 1,
                        branches: vec![
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 1,
                            },
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 500,
                            },
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "2",
                                taken: 1,
                            },
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "3",
                                taken: 0,
                            },
                            Branch {
                                line_number: 10,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 500,
                            },
                            Branch {
                                line_number: 10,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 10,
                                exception: false,
                                block: 0,
                                branch: "2",
                                taken: 0,
                            },
                            Branch {
                                line_number: 10,
                                exception: false,
                                block: 0,
                                branch: "3",
                                taken: 500,
                            },
                        ],
                        branches_found: 8,
                        branches_hit: 5,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 4,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 5,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 7,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 10,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 12,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 15,
                                execution_count: 500,
                                checksum: None,
                            },
                        ],
                        lines_hit: 5,
                        lines_found: 6,
                    },
                    Record {
                        test_name: "",
                        source_file_name: "/home/mariell/work/cov/samples/c/sample_test.c",
                        functions: vec![Function {
                            name: "main",
                            line_number_start: 4,
                            line_number_end: Some(20),
                            execution_count: 1,
                            aliases: vec![("main", 1)],
                        }],
                        functions_found: 1,
                        functions_hit: 1,
                        branches: vec![
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 500,
                            },
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 1,
                            },
                            Branch {
                                line_number: 7,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 7,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 500,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 1,
                            },
                        ],
                        branches_found: 6,
                        branches_hit: 4,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 4,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 5,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 6,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 7,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 8,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 9,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 13,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 14,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 15,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 16,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 19,
                                execution_count: 1,
                                checksum: None,
                            },
                        ],
                        lines_hit: 7,
                        lines_found: 11,
                    },
                ],
            },
        );
    }

    #[test]
    fn parses_valid_cpp_coverage() {
        let report = r#"
TN:
SF:/home/mariell/work/cov/samples/cpp/sample.cpp
FNL:0,3,3
FNA:0,501,_Z3minIiET_S0_S0_
FNL:1,5,5
FNA:1,1,_Z3maxIiET_S0_S0_
FNL:2,8,20
FNA:2,501,_ZN6sample5Adder3addEii
FNF:3
FNH:3
BRDA:3,0,0,501
BRDA:3,0,1,0
BRDA:5,0,0,0
BRDA:5,0,1,1
BRDA:9,0,0,1
BRDA:9,0,1,500
BRDA:9,0,2,1
BRDA:9,0,3,0
BRDA:9,0,4,1
BRDA:9,0,5,500
BRDA:14,0,0,500
BRDA:14,0,1,0
BRDA:14,0,2,0
BRDA:14,0,3,500
BRF:14
BRH:9
DA:3,501
DA:5,1
DA:8,501
DA:9,501
DA:11,1
DA:14,500
DA:16,0
DA:19,500
LF:8
LH:7
end_of_record
TN:
SF:/home/mariell/work/cov/samples/cpp/sample_test.cpp
FNL:0,4,21
FNA:0,1,main
FNF:1
FNH:1
BRDA:6,0,0,500
BRDA:6,0,1,1
BRDA:7,0,0,500
BRDA:7,e0,1,0
BRDA:8,0,0,0
BRDA:8,0,1,500
BRDA:9,0,0,-
BRDA:9,e0,1,-
BRDA:9,0,2,-
BRDA:9,e0,3,-
BRDA:9,0,4,-
BRDA:9,e0,5,-
BRDA:14,0,0,1
BRDA:14,e0,1,0
BRDA:15,0,0,0
BRDA:15,0,1,1
BRDA:16,0,0,-
BRDA:16,e0,1,-
BRDA:16,0,2,-
BRDA:16,e0,3,-
BRDA:16,0,4,-
BRDA:16,e0,5,-
BRF:22
BRH:6
DA:4,1
DA:6,501
DA:7,500
DA:8,500
DA:9,0
DA:10,0
DA:14,1
DA:15,1
DA:16,0
DA:17,0
DA:20,1
LF:11
LH:7
end_of_record
"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                record: vec![
                    Record {
                        test_name: "",
                        source_file_name: "/home/mariell/work/cov/samples/cpp/sample.cpp",
                        functions: vec![
                            Function {
                                name: "_Z3minIiET_S0_S0_",
                                line_number_start: 3,
                                line_number_end: Some(3),
                                execution_count: 501,
                                aliases: vec![("_Z3minIiET_S0_S0_", 501)],
                            },
                            Function {
                                name: "_Z3maxIiET_S0_S0_",
                                line_number_start: 5,
                                line_number_end: Some(5),
                                execution_count: 1,
                                aliases: vec![("_Z3maxIiET_S0_S0_", 1)],
                            },
                            Function {
                                name: "_ZN6sample5Adder3addEii",
                                line_number_start: 8,
                                line_number_end: Some(20),
                                execution_count: 501,
                                aliases: vec![("_ZN6sample5Adder3addEii", 501)],
                            },
                        ],
                        functions_hit: 3,
                        functions_found: 3,
                        branches: vec![
                            Branch {
                                line_number: 3,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 501,
                            },
                            Branch {
                                line_number: 3,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 5,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 1,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 1,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 500,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "2",
                                taken: 1,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "3",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "4",
                                taken: 1,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "5",
                                taken: 500,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 500,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "2",
                                taken: 0,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "3",
                                taken: 500,
                            },
                        ],
                        branches_found: 14,
                        branches_hit: 9,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 3,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 5,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 8,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 9,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 11,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 14,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 16,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 19,
                                execution_count: 500,
                                checksum: None,
                            },
                        ],
                        lines_hit: 7,
                        lines_found: 8,
                    },
                    Record {
                        test_name: "",
                        source_file_name: "/home/mariell/work/cov/samples/cpp/sample_test.cpp"
                            .into(),
                        functions: vec![Function {
                            name: "main",
                            line_number_start: 4,
                            line_number_end: Some(21),
                            execution_count: 1,
                            aliases: vec![("main", 1)],
                        }],
                        functions_found: 1,
                        functions_hit: 1,
                        branches: vec![
                            Branch {
                                line_number: 6,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 500,
                            },
                            Branch {
                                line_number: 6,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 1,
                            },
                            Branch {
                                line_number: 7,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 500,
                            },
                            Branch {
                                line_number: 7,
                                exception: true,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 8,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 8,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 500,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: true,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "2",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: true,
                                block: 0,
                                branch: "3",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 0,
                                branch: "4",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: true,
                                block: 0,
                                branch: "5",
                                taken: 0,
                            },
                            Branch {
                                line_number: 14,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 1,
                            },
                            Branch {
                                line_number: 14,
                                exception: true,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 15,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 15,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 1,
                            },
                            Branch {
                                line_number: 16,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 16,
                                exception: true,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 16,
                                exception: false,
                                block: 0,
                                branch: "2",
                                taken: 0,
                            },
                            Branch {
                                line_number: 16,
                                exception: true,
                                block: 0,
                                branch: "3",
                                taken: 0,
                            },
                            Branch {
                                line_number: 16,
                                exception: false,
                                block: 0,
                                branch: "4",
                                taken: 0,
                            },
                            Branch {
                                line_number: 16,
                                exception: true,
                                block: 0,
                                branch: "5",
                                taken: 0,
                            },
                        ],
                        branches_found: 22,
                        branches_hit: 6,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 4,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 6,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 7,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 8,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 9,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 10,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 14,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 15,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 16,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 17,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 20,
                                execution_count: 1,
                                checksum: None,
                            },
                        ],
                        lines_found: 11,
                        lines_hit: 7,
                    },
                ]
            }
        );
    }

    #[test]
    fn parses_valid_js_coverage() {
        let report = r#"
TN:
SF:helpers.js
FN:1,min
FN:5,max
FNF:2
FNH:2
FNDA:501,min
FNDA:1,max
DA:2,501
DA:6,1
DA:9,1
LF:3
LH:3
BRDA:2,0,0,501
BRDA:2,0,1,0
BRDA:6,1,0,0
BRDA:6,1,1,1
BRF:4
BRH:2
end_of_record
TN:
SF:index.js
FN:3,add
FNF:1
FNH:1
FNDA:501,add
DA:1,1
DA:4,501
DA:6,1
DA:9,500
DA:11,0
DA:14,500
DA:16,1
LF:7
LH:6
BRDA:4,0,0,1
BRDA:4,0,1,500
BRDA:4,1,0,501
BRDA:4,1,1,1
BRDA:9,2,0,0
BRDA:9,2,1,500
BRDA:9,3,0,500
BRDA:9,3,1,0
BRF:8
BRH:6
end_of_record
"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                record: vec![
                    Record {
                        test_name: "",
                        source_file_name: "helpers.js",
                        functions: vec![
                            Function {
                                name: "max",
                                line_number_start: 5,
                                line_number_end: None,
                                execution_count: 1,
                                aliases: vec![("max", 1)],
                            },
                            Function {
                                name: "min",
                                line_number_start: 1,
                                line_number_end: None,
                                execution_count: 501,
                                aliases: vec![("min", 501)],
                            },
                        ],
                        functions_found: 2,
                        functions_hit: 2,
                        branches: vec![
                            Branch {
                                line_number: 2,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 501,
                            },
                            Branch {
                                line_number: 2,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 0,
                            },
                            Branch {
                                line_number: 6,
                                exception: false,
                                block: 1,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 6,
                                exception: false,
                                block: 1,
                                branch: "1",
                                taken: 1,
                            },
                        ],
                        branches_found: 4,
                        branches_hit: 2,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 2,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 6,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 9,
                                execution_count: 1,
                                checksum: None,
                            },
                        ],
                        lines_hit: 3,
                        lines_found: 3,
                    },
                    Record {
                        test_name: "",
                        source_file_name: "index.js",
                        functions: vec![Function {
                            name: "add",
                            line_number_start: 3,
                            line_number_end: None,
                            execution_count: 501,
                            aliases: vec![("add", 501)],
                        }],
                        functions_hit: 1,
                        functions_found: 1,
                        branches: vec![
                            Branch {
                                line_number: 4,
                                exception: false,
                                block: 0,
                                branch: "0",
                                taken: 1,
                            },
                            Branch {
                                line_number: 4,
                                exception: false,
                                block: 0,
                                branch: "1",
                                taken: 500,
                            },
                            Branch {
                                line_number: 4,
                                exception: false,
                                block: 1,
                                branch: "0",
                                taken: 501,
                            },
                            Branch {
                                line_number: 4,
                                exception: false,
                                block: 1,
                                branch: "1",
                                taken: 1,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 2,
                                branch: "0",
                                taken: 0,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 2,
                                branch: "1",
                                taken: 500,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 3,
                                branch: "0",
                                taken: 500,
                            },
                            Branch {
                                line_number: 9,
                                exception: false,
                                block: 3,
                                branch: "1",
                                taken: 0,
                            },
                        ],
                        branches_found: 8,
                        branches_hit: 6,
                        mcdc: vec![],
                        modified_coverage_conditions_found: 0,
                        modified_coverage_conditions_hit: 0,
                        lines: vec![
                            CoveredLine {
                                line_number: 1,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 4,
                                execution_count: 501,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 6,
                                execution_count: 1,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 9,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 11,
                                execution_count: 0,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 14,
                                execution_count: 500,
                                checksum: None,
                            },
                            CoveredLine {
                                line_number: 16,
                                execution_count: 1,
                                checksum: None,
                            },
                        ],
                        lines_hit: 6,
                        lines_found: 7,
                    },
                ],
            },
        );
    }

    #[test]
    fn parses_valid_rust_coverage() {
        let report = r#"
SF:/home/mariell/work/cov/samples/rust/src/lib.rs
FN:13,_RNvCs25cOZmasxCc_6sample11called_once
FN:5,_RNvCs25cOZmasxCc_6sample12never_called
FN:1,_RNvCs25cOZmasxCc_6sample3add
FN:9,_RNvCs25cOZmasxCc_6sample6looped
FN:34,_RNvNtCs25cOZmasxCc_6sample5testss_11test_looped
FN:41,_RNvNtCs25cOZmasxCc_6sample5testss_16test_called_once
FN:29,_RNvNtCs25cOZmasxCc_6sample5testss_17test_never_called
FN:22,_RNvNtCs25cOZmasxCc_6sample5testss_8it_works
FNDA:1,_RNvCs25cOZmasxCc_6sample11called_once
FNDA:0,_RNvCs25cOZmasxCc_6sample12never_called
FNDA:1,_RNvCs25cOZmasxCc_6sample3add
FNDA:500,_RNvCs25cOZmasxCc_6sample6looped
FNDA:1,_RNvNtCs25cOZmasxCc_6sample5testss_11test_looped
FNDA:1,_RNvNtCs25cOZmasxCc_6sample5testss_16test_called_once
FNDA:0,_RNvNtCs25cOZmasxCc_6sample5testss_17test_never_called
FNDA:1,_RNvNtCs25cOZmasxCc_6sample5testss_8it_works
FNF:8
FNH:6
DA:1,1
DA:2,1
DA:3,1
DA:5,0
DA:6,0
DA:7,0
DA:9,500
DA:10,500
DA:11,500
DA:13,1
DA:14,1
DA:15,1
DA:22,1
DA:23,1
DA:24,1
DA:25,1
DA:29,0
DA:30,0
DA:31,0
DA:34,1
DA:35,501
DA:36,500
DA:37,500
DA:38,1
DA:41,1
DA:42,1
DA:43,1
BRF:0
BRH:0
LF:27
LH:21
end_of_record
"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                record: vec![Record {
                    test_name: "",
                    source_file_name: "/home/mariell/work/cov/samples/rust/src/lib.rs",
                    functions: vec![
                        Function {
                            name: "_RNvCs25cOZmasxCc_6sample11called_once",
                            line_number_start: 13,
                            line_number_end: None,
                            execution_count: 1,
                            aliases: vec![("_RNvCs25cOZmasxCc_6sample11called_once", 1)],
                        },
                        Function {
                            name: "_RNvCs25cOZmasxCc_6sample12never_called",
                            line_number_start: 5,
                            line_number_end: None,
                            execution_count: 0,
                            aliases: vec![("_RNvCs25cOZmasxCc_6sample12never_called", 0)],
                        },
                        Function {
                            name: "_RNvCs25cOZmasxCc_6sample3add",
                            line_number_start: 1,
                            line_number_end: None,
                            execution_count: 1,
                            aliases: vec![("_RNvCs25cOZmasxCc_6sample3add", 1)],
                        },
                        Function {
                            name: "_RNvCs25cOZmasxCc_6sample6looped",
                            line_number_start: 9,
                            line_number_end: None,
                            execution_count: 500,
                            aliases: vec![("_RNvCs25cOZmasxCc_6sample6looped", 500)],
                        },
                        Function {
                            name: "_RNvNtCs25cOZmasxCc_6sample5testss_11test_looped",
                            line_number_start: 34,
                            line_number_end: None,
                            execution_count: 1,
                            aliases: vec![("_RNvNtCs25cOZmasxCc_6sample5testss_11test_looped", 1)],
                        },
                        Function {
                            name: "_RNvNtCs25cOZmasxCc_6sample5testss_16test_called_once",
                            line_number_start: 41,
                            line_number_end: None,
                            execution_count: 1,
                            aliases: vec![(
                                "_RNvNtCs25cOZmasxCc_6sample5testss_16test_called_once",
                                1
                            )],
                        },
                        Function {
                            name: "_RNvNtCs25cOZmasxCc_6sample5testss_17test_never_called",
                            line_number_start: 29,
                            line_number_end: None,
                            execution_count: 0,
                            aliases: vec![(
                                "_RNvNtCs25cOZmasxCc_6sample5testss_17test_never_called",
                                0
                            )],
                        },
                        Function {
                            name: "_RNvNtCs25cOZmasxCc_6sample5testss_8it_works",
                            line_number_start: 22,
                            line_number_end: None,
                            execution_count: 1,
                            aliases: vec![("_RNvNtCs25cOZmasxCc_6sample5testss_8it_works", 1)],
                        },
                    ],
                    functions_found: 8,
                    functions_hit: 6,
                    branches: vec![],
                    branches_found: 0,
                    branches_hit: 0,
                    mcdc: vec![],
                    modified_coverage_conditions_found: 0,
                    modified_coverage_conditions_hit: 0,
                    lines: vec![
                        CoveredLine {
                            line_number: 1,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 2,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 3,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 5,
                            execution_count: 0,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 6,
                            execution_count: 0,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 7,
                            execution_count: 0,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 9,
                            execution_count: 500,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 10,
                            execution_count: 500,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 11,
                            execution_count: 500,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 13,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 14,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 15,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 22,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 23,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 24,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 25,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 29,
                            execution_count: 0,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 30,
                            execution_count: 0,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 31,
                            execution_count: 0,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 34,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 35,
                            execution_count: 501,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 36,
                            execution_count: 500,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 37,
                            execution_count: 500,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 38,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 41,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 42,
                            execution_count: 1,
                            checksum: None,
                        },
                        CoveredLine {
                            line_number: 43,
                            execution_count: 1,
                            checksum: None,
                        },
                    ],
                    lines_hit: 21,
                    lines_found: 27,
                }],
            },
        );
    }
}
