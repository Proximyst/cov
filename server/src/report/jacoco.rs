use super::InvalidReport;
use serde::Deserialize;
use std::str::FromStr;

/// A JaCoCo report detailing Java code coverage.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Report {
    /// The name of the report.
    /// This is usually the same as the artifactId or name of the Maven/Gradle project.
    #[serde(rename = "@name")]
    pub name: String,
    /// The counters detailing aggregate coverage of the entire report.
    #[serde(rename = "counter")]
    pub counters: Vec<Counter>,
    /// The packages in the report, with details on each.
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

impl FromStr for Report {
    type Err = InvalidReport;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|_| InvalidReport)
    }
}

/// A singular counter.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Counter {
    /// The type of counter. Each counter type can only appear once per report.
    #[serde(rename = "@type")]
    pub typ: CounterType,
    /// The number of missed items. The semantic meaning depends on the type.
    #[serde(rename = "@missed")]
    pub missed: u32,
    /// The number of covered items. The semantic meaning depends on the type.
    #[serde(rename = "@covered")]
    pub covered: u32,
}

/// A type of counter. This is used to differentiate the semantic meaning of the missed and covered fields.
#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq)]
pub enum CounterType {
    /// TODO: Bytecode instructions??
    #[serde(rename = "INSTRUCTION")]
    Instruction,
    /// Counts the executable lines.
    ///
    /// Lines that only contain braces, are empty, comments, or similar are not counted, as there is no code to execute.
    #[serde(rename = "LINE")]
    ExecutableLines,
    /// TODO: No idea
    #[serde(rename = "COMPLEXITY")]
    Complexity,
    /// Counts the methods. All methods are executable, hence counted.
    #[serde(rename = "METHOD")]
    Method,
    /// Counts the classes. All classes are counted.
    #[serde(rename = "CLASS")]
    Class,
}

/// Encapsulates a single package within a report. Subpackages are separate entries.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Package {
    /// The name of the package. Uses slashes, like how JVM byte code would represent packages.
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "counter")]
    pub counters: Vec<Counter>,
    #[serde(rename = "class")]
    pub classes: Vec<Class>,
    #[serde(rename = "sourcefile")]
    pub source_files: Vec<SourceFile>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Class {
    /// The name of the class. Uses slashes, like how JVM byte code would represent classes.
    ///
    /// This is prefixed by the package, so `dev/mardroemmar/cov/sample/Sample` would be the full name for `Sample` in `dev.mardroemmar.cov.sample`.
    #[serde(rename = "@name")]
    pub name: String,
    /// The name of the source file this class is defined in. This does not include the full path, i.e. `Sample.java` would be the full name for `Sample` in `dev.mardroemmar.cov.sample`.
    #[serde(rename = "@sourcefilename")]
    pub file_name: String,
    #[serde(rename = "method")]
    pub methods: Vec<Method>,
    #[serde(rename = "counter")]
    pub counters: Vec<Counter>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Method {
    /// The method's name. Can include unicode characters if the underlying language supports it (e.g. Kotlin).
    #[serde(rename = "@name")]
    pub name: String,
    /// The byte code descriptor of this method. E.g. `()V` for a `void()` method, or `(Z)V` for a `void(boolean)` method.
    #[serde(rename = "@desc")]
    pub desc: String,
    /// The line at which this method's executable code starts.
    /// This means this is not the method definition itself, but the first line within that would be counted for coverage.
    #[serde(rename = "@line")]
    pub line: u32,
    #[serde(rename = "counter")]
    pub counters: Vec<Counter>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SourceFile {
    /// The name of the source file. This does not include the full path, i.e. `Sample.java` would be the full name for `Sample` in `dev.mardroemmar.cov.sample`.
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "line")]
    pub lines: Vec<Line>,
    #[serde(rename = "counter")]
    pub counters: Vec<Counter>,
}

/// Statistics about a single line in a source file.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Line {
    /// The line number within a file.
    #[serde(rename = "@nr")]
    pub number: u32,
    /// TODO: Missed??
    #[serde(rename = "@mi")]
    pub mi: u32,
    /// TODO: No idea
    #[serde(rename = "@ci")]
    pub ci: u32,
    /// TODO: Methods??
    #[serde(rename = "@mb")]
    pub mb: u32,
    /// TODO: No idea
    #[serde(rename = "@cb")]
    pub cb: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn parse_valid_report() {
        let report = r#"
            <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <!DOCTYPE report PUBLIC "-//JACOCO//DTD Report 1.1//EN" "report.dtd">
            <report name="sample">
              <sessioninfo id="mariell-work-3697872d" start="1741637758834" dump="1741637759283"/>
              <sessioninfo id="mariell-work-fd143e18" start="1741637850712" dump="1741637851114"/>
              <sessioninfo id="mariell-work-79a40044" start="1741638003729" dump="1741638004166"/>
              <sessioninfo id="mariell-work-8ab135f" start="1741638120056" dump="1741638120482"/>
              <sessioninfo id="mariell-work-18e4cc2d" start="1741638177581" dump="1741638177971"/>
              <sessioninfo id="mariell-work-a9ddb881" start="1741638244510" dump="1741638244940"/>
              <sessioninfo id="mariell-work-d947464b" start="1741638493831" dump="1741638494238"/>
              <sessioninfo id="mariell-work-4a4d9b5" start="1741638523312" dump="1741638523711"/>
              <package name="dev/mardroemmar/cov/sample/uncalled">
                <class name="dev/mardroemmar/cov/sample/uncalled/UncalledPackage" sourcefilename="UncalledPackage.java">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="3" covered="0"/>
                    <counter type="LINE" missed="1" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <method name="neverCalled" desc="()V" line="6">
                    <counter type="INSTRUCTION" missed="4" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="7" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <sourcefile name="UncalledPackage.java">
                  <line nr="3" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="6" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="7" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="7" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </sourcefile>
                <counter type="INSTRUCTION" missed="7" covered="0"/>
                <counter type="LINE" missed="3" covered="0"/>
                <counter type="COMPLEXITY" missed="2" covered="0"/>
                <counter type="METHOD" missed="2" covered="0"/>
                <counter type="CLASS" missed="1" covered="0"/>
              </package>
              <package name="dev/mardroemmar/cov/sample">
                <class name="dev/mardroemmar/cov/sample/Sample" sourcefilename="Sample.java">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="0" covered="3"/>
                    <counter type="LINE" missed="0" covered="1"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <method name="neverCalled" desc="()V" line="6">
                    <counter type="INSTRUCTION" missed="4" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <method name="calledOnce" desc="()V" line="10">
                    <counter type="INSTRUCTION" missed="0" covered="4"/>
                    <counter type="LINE" missed="0" covered="2"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <method name="calledManyTimes" desc="(Ljava/lang/String;)V" line="14">
                    <counter type="INSTRUCTION" missed="0" covered="19"/>
                    <counter type="LINE" missed="0" covered="5"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <counter type="INSTRUCTION" missed="4" covered="26"/>
                  <counter type="LINE" missed="2" covered="8"/>
                  <counter type="COMPLEXITY" missed="1" covered="3"/>
                  <counter type="METHOD" missed="1" covered="3"/>
                  <counter type="CLASS" missed="0" covered="1"/>
                </class>
                <class name="dev/mardroemmar/cov/sample/UncalledClass" sourcefilename="UncalledClass.java">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="3" covered="0"/>
                    <counter type="LINE" missed="1" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <method name="neverCalled" desc="()V" line="6">
                    <counter type="INSTRUCTION" missed="4" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="7" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <sourcefile name="Sample.java">
                  <line nr="3" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="6" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="7" mi="1" ci="0" mb="0" cb="0"/>
                  <line nr="10" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="11" mi="0" ci="1" mb="0" cb="0"/>
                  <line nr="14" mi="0" ci="9" mb="0" cb="0"/>
                  <line nr="15" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="16" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="17" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="18" mi="0" ci="1" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="4" covered="26"/>
                  <counter type="LINE" missed="2" covered="8"/>
                  <counter type="COMPLEXITY" missed="1" covered="3"/>
                  <counter type="METHOD" missed="1" covered="3"/>
                  <counter type="CLASS" missed="0" covered="1"/>
                </sourcefile>
                <sourcefile name="UncalledClass.java">
                  <line nr="3" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="6" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="7" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="7" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </sourcefile>
                <counter type="INSTRUCTION" missed="11" covered="26"/>
                <counter type="LINE" missed="5" covered="8"/>
                <counter type="COMPLEXITY" missed="3" covered="3"/>
                <counter type="METHOD" missed="3" covered="3"/>
                <counter type="CLASS" missed="1" covered="1"/>
              </package>
              <counter type="INSTRUCTION" missed="18" covered="26"/>
              <counter type="LINE" missed="8" covered="8"/>
              <counter type="COMPLEXITY" missed="5" covered="3"/>
              <counter type="METHOD" missed="5" covered="3"/>
              <counter type="CLASS" missed="2" covered="1"/>
            </report>"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                name: "sample".into(),
                counters: vec![
                    Counter {
                        typ: CounterType::Instruction,
                        missed: 18,
                        covered: 26,
                    },
                    Counter {
                        typ: CounterType::ExecutableLines,
                        missed: 8,
                        covered: 8,
                    },
                    Counter {
                        typ: CounterType::Complexity,
                        missed: 5,
                        covered: 3,
                    },
                    Counter {
                        typ: CounterType::Method,
                        missed: 5,
                        covered: 3,
                    },
                    Counter {
                        typ: CounterType::Class,
                        missed: 2,
                        covered: 1,
                    },
                ],
                packages: vec![
                    Package {
                        name: "dev/mardroemmar/cov/sample/uncalled".into(),
                        classes: vec![Class {
                            name: "dev/mardroemmar/cov/sample/uncalled/UncalledPackage".into(),
                            file_name: "UncalledPackage.java".into(),
                            methods: vec![
                                Method {
                                    name: "<init>".into(),
                                    desc: "()V".into(),
                                    line: 3,
                                    counters: vec![
                                        Counter {
                                            typ: CounterType::Instruction,
                                            missed: 3,
                                            covered: 0,
                                        },
                                        Counter {
                                            typ: CounterType::ExecutableLines,
                                            missed: 1,
                                            covered: 0,
                                        },
                                        Counter {
                                            typ: CounterType::Complexity,
                                            missed: 1,
                                            covered: 0,
                                        },
                                        Counter {
                                            typ: CounterType::Method,
                                            missed: 1,
                                            covered: 0,
                                        },
                                    ],
                                },
                                Method {
                                    name: "neverCalled".into(),
                                    desc: "()V".into(),
                                    line: 6,
                                    counters: vec![
                                        Counter {
                                            typ: CounterType::Instruction,
                                            missed: 4,
                                            covered: 0,
                                        },
                                        Counter {
                                            typ: CounterType::ExecutableLines,
                                            missed: 2,
                                            covered: 0,
                                        },
                                        Counter {
                                            typ: CounterType::Complexity,
                                            missed: 1,
                                            covered: 0,
                                        },
                                        Counter {
                                            typ: CounterType::Method,
                                            missed: 1,
                                            covered: 0,
                                        },
                                    ],
                                },
                            ],
                            counters: vec![
                                Counter {
                                    typ: CounterType::Instruction,
                                    missed: 7,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::ExecutableLines,
                                    missed: 3,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::Complexity,
                                    missed: 2,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::Method,
                                    missed: 2,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::Class,
                                    missed: 1,
                                    covered: 0,
                                },
                            ],
                        }],
                        source_files: vec![SourceFile {
                            name: "UncalledPackage.java".into(),
                            lines: vec![
                                Line {
                                    number: 3,
                                    mi: 3,
                                    ci: 0,
                                    mb: 0,
                                    cb: 0,
                                },
                                Line {
                                    number: 6,
                                    mi: 3,
                                    ci: 0,
                                    mb: 0,
                                    cb: 0,
                                },
                                Line {
                                    number: 7,
                                    mi: 1,
                                    ci: 0,
                                    mb: 0,
                                    cb: 0,
                                },
                            ],
                            counters: vec![
                                Counter {
                                    typ: CounterType::Instruction,
                                    missed: 7,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::ExecutableLines,
                                    missed: 3,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::Complexity,
                                    missed: 2,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::Method,
                                    missed: 2,
                                    covered: 0,
                                },
                                Counter {
                                    typ: CounterType::Class,
                                    missed: 1,
                                    covered: 0,
                                },
                            ],
                        }],
                        counters: vec![
                            Counter {
                                typ: CounterType::Instruction,
                                missed: 7,
                                covered: 0,
                            },
                            Counter {
                                typ: CounterType::ExecutableLines,
                                missed: 3,
                                covered: 0,
                            },
                            Counter {
                                typ: CounterType::Complexity,
                                missed: 2,
                                covered: 0,
                            },
                            Counter {
                                typ: CounterType::Method,
                                missed: 2,
                                covered: 0,
                            },
                            Counter {
                                typ: CounterType::Class,
                                missed: 1,
                                covered: 0,
                            },
                        ],
                    },
                    Package {
                        name: "dev/mardroemmar/cov/sample".into(),
                        counters: vec![
                            Counter {
                                typ: CounterType::Instruction,
                                missed: 11,
                                covered: 26,
                            },
                            Counter {
                                typ: CounterType::ExecutableLines,
                                missed: 5,
                                covered: 8,
                            },
                            Counter {
                                typ: CounterType::Complexity,
                                missed: 3,
                                covered: 3,
                            },
                            Counter {
                                typ: CounterType::Method,
                                missed: 3,
                                covered: 3,
                            },
                            Counter {
                                typ: CounterType::Class,
                                missed: 1,
                                covered: 1,
                            },
                        ],
                        classes: vec![
                            Class {
                                name: "dev/mardroemmar/cov/sample/Sample".into(),
                                file_name: "Sample.java".into(),
                                methods: vec![
                                    Method {
                                        name: "<init>".into(),
                                        desc: "()V".into(),
                                        line: 3,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 3,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 1,
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 0,
                                                covered: 1,
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 0,
                                                covered: 1,
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "neverCalled".into(),
                                        desc: "()V".into(),
                                        line: 6,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 4,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 2,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 1,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 1,
                                                covered: 0,
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "calledOnce".into(),
                                        desc: "()V".into(),
                                        line: 10,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 4,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 2,
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 0,
                                                covered: 1,
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 0,
                                                covered: 1,
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "calledManyTimes".into(),
                                        desc: "(Ljava/lang/String;)V".into(),
                                        line: 14,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 19,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 5,
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 0,
                                                covered: 1,
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 0,
                                                covered: 1,
                                            },
                                        ],
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 4,
                                        covered: 26,
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 2,
                                        covered: 8,
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 1,
                                        covered: 3,
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 1,
                                        covered: 3,
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 0,
                                        covered: 1,
                                    },
                                ],
                            },
                            Class {
                                name: "dev/mardroemmar/cov/sample/UncalledClass".into(),
                                file_name: "UncalledClass.java".into(),
                                methods: vec![
                                    Method {
                                        name: "<init>".into(),
                                        desc: "()V".into(),
                                        line: 3,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 3,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 1,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 1,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 1,
                                                covered: 0,
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "neverCalled".into(),
                                        desc: "()V".into(),
                                        line: 6,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 4,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 2,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 1,
                                                covered: 0,
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 1,
                                                covered: 0,
                                            },
                                        ],
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 7,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 3,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 2,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 2,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 1,
                                        covered: 0,
                                    },
                                ],
                            },
                        ],
                        source_files: vec![
                            SourceFile {
                                name: "Sample.java".into(),
                                lines: vec![
                                    Line {
                                        number: 3,
                                        mi: 0,
                                        ci: 3,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 6,
                                        mi: 3,
                                        ci: 0,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 7,
                                        mi: 1,
                                        ci: 0,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 10,
                                        mi: 0,
                                        ci: 3,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 11,
                                        mi: 0,
                                        ci: 1,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 14,
                                        mi: 0,
                                        ci: 9,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 15,
                                        mi: 0,
                                        ci: 3,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 16,
                                        mi: 0,
                                        ci: 3,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 17,
                                        mi: 0,
                                        ci: 3,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 18,
                                        mi: 0,
                                        ci: 1,
                                        mb: 0,
                                        cb: 0,
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 4,
                                        covered: 26,
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 2,
                                        covered: 8,
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 1,
                                        covered: 3,
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 1,
                                        covered: 3,
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 0,
                                        covered: 1,
                                    },
                                ],
                            },
                            SourceFile {
                                name: "UncalledClass.java".into(),
                                lines: vec![
                                    Line {
                                        number: 3,
                                        mi: 3,
                                        ci: 0,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 6,
                                        mi: 3,
                                        ci: 0,
                                        mb: 0,
                                        cb: 0,
                                    },
                                    Line {
                                        number: 7,
                                        mi: 1,
                                        ci: 0,
                                        mb: 0,
                                        cb: 0,
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 7,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 3,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 2,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 2,
                                        covered: 0,
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 1,
                                        covered: 0,
                                    },
                                ],
                            },
                        ],
                    }
                ],
            }
        );
    }
}
