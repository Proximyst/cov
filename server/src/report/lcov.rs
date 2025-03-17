//! This format is described in the [Lcov documentation](https://github.com/linux-test-project/lcov/blob/38a82d5d03c449b4253223c111aef1c36e46d5db/man/geninfo.1#L1370).
//! It can also be accessed via `man geninfo -1` on a system with `lcov` installed; find the `TRACEFILE FORMAT` section.

use super::InvalidReport;
use std::str::FromStr;
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{alt, opt, preceded, terminated},
    token::{literal, rest, take_till},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {}

impl FromStr for Report {
    type Err = InvalidReport;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let s = &mut s;
        let report = parse_report(s).map_err(|_| InvalidReport)?;
        if s.is_empty() {
            Ok(report)
        } else {
            Err(InvalidReport)
        }
    }
}

fn parse_report(s: &mut &str) -> Result<Report> {
    todo!()
}

/// One individual test and its coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Test {
    /// The name of the test. Can be an empty string.
    ///
    /// This is the `TN` field in the LCOV report.
    pub test_name: String,

    /// The name of the source file.
    /// Can be relative or absolute. If relative, it is assumed to be relative to the root of the repository.
    ///
    /// This is the `SF` field in the LCOV report.
    pub source_file_name: String,
}

/// Reads and discards a comment line.
fn discard_comment(s: &mut &str) -> Result<()> {
    let _ = literal("#").parse_next(s)?;
    let _ = rest.parse_next(s)?;
    Ok(())
}

/// Reads a test name field. Could be empty.
fn parse_test_name(s: &mut &str) -> Result<String> {
    let _ = literal("TN:").parse_next(s)?;
    let test_name = rest.parse_next(s)?.into();
    Ok(test_name)
}

/// Reads a source file name field.
fn parse_source_file_name(s: &mut &str) -> Result<String> {
    let _ = literal("SF:").parse_next(s)?;
    let source_file_name = rest.parse_next(s)?.into();
    Ok(source_file_name)
}

/// Reads a source code version ID.
fn parse_source_code_version(s: &mut &str) -> Result<String> {
    let _ = literal("VER:").parse_next(s)?;
    let source_code_version = rest.parse_next(s)?.into();
    Ok(source_code_version)
}

/// `FNL:<index>,<line number start>[,<line number end>]`
struct ModernFunctionLeader {
    pub index: u32,
    pub line_number: u32,
    pub line_number_end: Option<u32>,
}

/// Reads a function leader.
fn parse_modern_function_leader(s: &mut &str) -> Result<ModernFunctionLeader> {
    let _ = literal("FNL:").parse_next(s)?;
    let index = terminated(dec_uint, ",").parse_next(s)?;
    let line_number = dec_uint.parse_next(s)?;
    let line_number_end = opt(preceded(",", dec_uint)).parse_next(s)?;
    Ok(ModernFunctionLeader {
        index,
        line_number,
        line_number_end,
    })
}

/// `FNA:<function index>,<execution count>,<name>`
struct FunctionAlias {
    pub index: u32,
    pub execution_count: u32,
    pub name: String,
}

fn parse_function_alias(s: &mut &str) -> Result<FunctionAlias> {
    let _ = literal("FNA:").parse_next(s)?;
    let index = terminated(dec_uint, ",").parse_next(s)?;
    let execution_count = terminated(dec_uint, ",").parse_next(s)?;
    let name = rest.parse_next(s)?.into();
    Ok(FunctionAlias {
        index,
        execution_count,
        name,
    })
}

/// `FN:<line number start>[,<line number end>],<function name>`
struct LegacyFunctionLeader {
    pub line_number: u32,
    pub line_number_end: Option<u32>,
    pub function_name: String,
}

/// Reads a legacy function leader.
fn parse_legacy_function_leader(s: &mut &str) -> Result<LegacyFunctionLeader> {
    let _ = literal("FN:").parse_next(s)?;
    let line_number = terminated(dec_uint, ",").parse_next(s)?;
    let line_number_end = opt(terminated(dec_uint, ",")).parse_next(s)?;
    let function_name = rest.parse_next(s)?.into();
    Ok(LegacyFunctionLeader {
        line_number,
        line_number_end,
        function_name,
    })
}

/// `FNDA:<execution count>,<function index>`
struct LegacyFunctionData {
    pub execution_count: u32,
    pub function_index: u32,
}

fn parse_legacy_function_data(s: &mut &str) -> Result<LegacyFunctionData> {
    let _ = literal("FNDA:").parse_next(s)?;
    let execution_count = terminated(dec_uint, ",").parse_next(s)?;
    let function_index = dec_uint.parse_next(s)?;
    Ok(LegacyFunctionData {
        execution_count,
        function_index,
    })
}

fn parse_functions_found(s: &mut &str) -> Result<u32> {
    let _ = literal("FNF:").parse_next(s)?;
    let functions_found = dec_uint.parse_next(s)?;
    Ok(functions_found)
}

fn parse_functions_hit(s: &mut &str) -> Result<u32> {
    let _ = literal("FNH:").parse_next(s)?;
    let functions_found = dec_uint.parse_next(s)?;
    Ok(functions_found)
}

/// `BRDA:<line number>,[<exception>]<block>,<branch>,<taken>`
struct BRDA {
    pub line_number: u32,
    pub exception: bool,
    pub block: u32,
    pub branch: String,
    pub taken: u32,
}

fn parse_branch_coverage(s: &mut &str) -> Result<BRDA> {
    let _ = literal("BRDA:").parse_next(s)?;
    let line_number = terminated(dec_uint, ",").parse_next(s)?;
    let exception = opt("e").map(|o| o.is_some()).parse_next(s)?;
    let block = terminated(dec_uint, ",").parse_next(s)?;
    let branch = take_till(0.., '0'..='9').parse_next(s)?.into();
    let taken = dec_uint.parse_next(s)?;
    Ok(BRDA {
        line_number,
        exception,
        block,
        branch,
        taken,
    })
}

fn parse_branches_found(s: &mut &str) -> Result<u32> {
    let _ = literal("BRF:").parse_next(s)?;
    let branches_found = dec_uint.parse_next(s)?;
    Ok(branches_found)
}

fn parse_branches_hit(s: &mut &str) -> Result<u32> {
    let _ = literal("BRH:").parse_next(s)?;
    let branches_hit = dec_uint.parse_next(s)?;
    Ok(branches_hit)
}

/// `MCDC:<line number>,<group size>,<sense>,<taken>,<index>,<expression>`
struct MCDC {
    pub line_number: u32,
    pub group_size: u32,
    pub sense: bool,
    /// Some tools treat this as a boolean, others as a counter.
    pub taken: u32,
    /// The index is at least `0`, and at most `group_size - 1`.
    pub index: u32,
    pub expression: String,
}

fn parse_mcdc(s: &mut &str) -> Result<MCDC> {
    let _ = literal("MCDC:").parse_next(s)?;
    let line_number = terminated(dec_uint, ",").parse_next(s)?;
    let group_size = terminated(dec_uint, ",").parse_next(s)?;
    let sense = terminated(alt(("t", "f")), ",")
        .map(|s| s == "t")
        .parse_next(s)?;
    let taken = terminated(dec_uint, ",").parse_next(s)?;
    let index = terminated(dec_uint, ",").parse_next(s)?;
    let expression = rest.parse_next(s)?.into();
    Ok(MCDC {
        line_number,
        group_size,
        sense,
        taken,
        index,
        expression,
    })
}

fn parse_mcdc_found(s: &mut &str) -> Result<u32> {
    let _ = literal("MRF:").parse_next(s)?;
    let mcdc_found = dec_uint.parse_next(s)?;
    Ok(mcdc_found)
}

fn parse_mcdc_hit(s: &mut &str) -> Result<u32> {
    let _ = literal("MRH:").parse_next(s)?;
    let mcdc_hit = dec_uint.parse_next(s)?;
    Ok(mcdc_hit)
}

/// `DA:<line number>,<execution count>[,<checksum>]`
struct DA {
    pub line_number: u32,
    pub execution_count: u32,
    pub checksum: Option<String>,
}

fn parse_da(s: &mut &str) -> Result<DA> {
    let _ = literal("DA:").parse_next(s)?;
    let line_number = terminated(dec_uint, ",").parse_next(s)?;
    let execution_count = dec_uint.parse_next(s)?;
    let checksum = opt(preceded(",", rest))
        .map(|s| s.map(|s: &str| s.to_owned()))
        .parse_next(s)?;
    Ok(DA {
        line_number,
        execution_count,
        checksum,
    })
}

fn parse_lines_hit(s: &mut &str) -> Result<u32> {
    let _ = literal("LH:").parse_next(s)?;
    let lines_hit = dec_uint.parse_next(s)?;
    Ok(lines_hit)
}

fn parse_lines_found(s: &mut &str) -> Result<u32> {
    let _ = literal("LF:").parse_next(s)?;
    let lines_found = dec_uint.parse_next(s)?;
    Ok(lines_found)
}

fn parse_end_of_record(s: &mut &str) -> Result<()> {
    let _ = literal("end_of_record").parse_next(s)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_valid_c_coverage() {
        let _report = r#"
            TN:
            SF:/home/mariell/work/cov/samples/c/helpers.c
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
    }

    #[test]
    fn parses_valid_cpp_coverage() {
        let _report = r#"
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
    }

    #[test]
    fn parses_valid_js_coverage() {
        let _report = r#"
            TN:
            SF:/home/mariell/work/cov/samples/c/helpers.c
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
    }
}
