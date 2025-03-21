use serde::Deserialize;

/// A JaCoCo report detailing Java code coverage.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Report {
    /// The name of the report.
    /// This is usually the same as the artifactId or name of the Maven/Gradle project.
    #[serde(rename = "@name")]
    pub name: String,
    /// The counters detailing aggregate coverage of the entire report.
    #[serde(rename = "counter", default)]
    pub counters: Vec<Counter>,
    /// The packages in the report, with details on each.
    #[serde(rename = "package", default)]
    pub packages: Vec<Package>,
}

impl Report {
    pub fn from_str(s: &str) -> Result<Self, quick_xml::de::DeError> {
        quick_xml::de::from_str(s)
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
    /// If this is the default package (i.e. one that is not declared), this will be an empty string.
    #[serde(rename = "@name")]
    pub name: String,
    /// The counters for this package.
    #[serde(rename = "counter", default)]
    pub counters: Vec<Counter>,
    /// The classes within this package.
    #[serde(rename = "class", default)]
    pub classes: Vec<Class>,
    /// The source files within this package.
    #[serde(rename = "sourcefile", default)]
    pub source_files: Vec<SourceFile>,
}

/// A single class within a package.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Class {
    /// The name of the class. Uses slashes, like how JVM byte code would represent classes.
    ///
    /// This is prefixed by the package, so `dev/mardroemmar/cov/sample/Sample` would be the full name for `Sample` in `dev.mardroemmar.cov.sample`.
    ///
    /// Some values are special, e.g. `<init>` for constructors and `<clinit>` for `static` blocks.
    #[serde(rename = "@name")]
    pub name: String,
    /// The name of the source file this class is defined in. This does not include the full path, i.e. `Sample.java` would be the full name for `Sample` in `dev.mardroemmar.cov.sample`.
    #[serde(rename = "@sourcefilename")]
    pub file_name: String,
    /// The methods within this class.
    #[serde(rename = "method", default)]
    pub methods: Vec<Method>,
    /// The counters for this class.
    #[serde(rename = "counter", default)]
    pub counters: Vec<Counter>,
}

/// A single method within a class.
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
    /// The counters for this method.
    #[serde(rename = "counter", default)]
    pub counters: Vec<Counter>,
}

/// A single raw source code file.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SourceFile {
    /// The name of the source file. This does not include the full path, i.e. `Sample.java` would be the full name for `Sample` in `dev.mardroemmar.cov.sample`.
    #[serde(rename = "@name")]
    pub name: String,
    /// The individual lines in this source file.
    #[serde(rename = "line", default)]
    pub lines: Vec<Line>,
    /// The counters for this source file.
    #[serde(rename = "counter", default)]
    pub counters: Vec<Counter>,
}

/// Statistics about a single line in a source file.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Line {
    /// The line number within a file.
    #[serde(rename = "@nr")]
    pub number: u32,
    /// Of the instructions in this line, how many of them were missed across the entire suite?
    /// This acts as a boolean on each instruction: if called, it is permanently known as hit.
    /// This is the opposite counter of `hit_calls`.
    ///
    /// The total number of instructions in this line is the sum of `missed_calls` and `hit_calls`.
    #[serde(rename = "@mi")]
    pub missed_calls: u32,
    /// Of the instructions in this line, how many of them were hit across the entire suite?
    /// This is the opposite counter of `missed_calls`.
    #[serde(rename = "@ci")]
    pub hit_calls: u32,
    /// Of all the branches in this line, how many of them were missed across the entire suite?
    /// This acts as a boolean on each branch: if called, it is permanently known as hit.
    /// This is the opposite counter of `called_branches`.
    ///
    /// The total number of branches in this line is the sum of `missed_branches` and `called_branches`.
    #[serde(rename = "@mb")]
    pub missed_branches: u32,
    /// Of all the branches in this line, how many of them were hit across the entire suite?
    /// This is the opposite counter of `missed_branches`.
    #[serde(rename = "@cb")]
    pub called_branches: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
                                    missed_calls: 3,
                                    hit_calls: 0,
                                    missed_branches: 0,
                                    called_branches: 0,
                                },
                                Line {
                                    number: 6,
                                    missed_calls: 3,
                                    hit_calls: 0,
                                    missed_branches: 0,
                                    called_branches: 0,
                                },
                                Line {
                                    number: 7,
                                    missed_calls: 1,
                                    hit_calls: 0,
                                    missed_branches: 0,
                                    called_branches: 0,
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
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 6,
                                        missed_calls: 3,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 7,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 10,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 11,
                                        missed_calls: 0,
                                        hit_calls: 1,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 14,
                                        missed_calls: 0,
                                        hit_calls: 9,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 15,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 16,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 17,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 18,
                                        missed_calls: 0,
                                        hit_calls: 1,
                                        missed_branches: 0,
                                        called_branches: 0,
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
                                        missed_calls: 3,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 6,
                                        missed_calls: 3,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 7,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
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

    #[test]
    fn parse_kotlin_report() {
        let report = r#"
            <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <!DOCTYPE report PUBLIC "-//JACOCO//DTD Report 1.1//EN" "report.dtd">
            <report name="sample">
              <sessioninfo id="mariell-work-702ea103" start="1742237164605" dump="1742237165025"/>
              <package name="">
                <class name="Sample" sourcefilename="Sample.kt">
                  <method name="&lt;init&gt;" desc="()V" line="2">
                    <counter type="INSTRUCTION" missed="3" covered="0"/>
                    <counter type="LINE" missed="1" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="3" covered="0"/>
                  <counter type="LINE" missed="1" covered="0"/>
                  <counter type="COMPLEXITY" missed="1" covered="0"/>
                  <counter type="METHOD" missed="1" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <sourcefile name="Sample.kt">
                  <line nr="2" mi="3" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="3" covered="0"/>
                  <counter type="LINE" missed="1" covered="0"/>
                  <counter type="COMPLEXITY" missed="1" covered="0"/>
                  <counter type="METHOD" missed="1" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </sourcefile>
                <counter type="INSTRUCTION" missed="3" covered="0"/>
                <counter type="LINE" missed="1" covered="0"/>
                <counter type="COMPLEXITY" missed="1" covered="0"/>
                <counter type="METHOD" missed="1" covered="0"/>
                <counter type="CLASS" missed="1" covered="0"/>
              </package>
              <package name="dev/mardroemmar/cov/sample/uncalled">
                <class name="dev/mardroemmar/cov/sample/uncalled/IAlsoHaveAnObject" sourcefilename="uncalledPackage.kt"/>
                <class name="dev/mardroemmar/cov/sample/uncalled/UncalledPackageKt" sourcefilename="uncalledPackage.kt">
                  <method name="neverCalled" desc="()V" line="4">
                    <counter type="INSTRUCTION" missed="5" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="5" covered="0"/>
                  <counter type="LINE" missed="2" covered="0"/>
                  <counter type="COMPLEXITY" missed="1" covered="0"/>
                  <counter type="METHOD" missed="1" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <sourcefile name="uncalledPackage.kt">
                  <line nr="4" mi="4" ci="0" mb="0" cb="0"/>
                  <line nr="5" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="5" covered="0"/>
                  <counter type="LINE" missed="2" covered="0"/>
                  <counter type="COMPLEXITY" missed="1" covered="0"/>
                  <counter type="METHOD" missed="1" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </sourcefile>
                <counter type="INSTRUCTION" missed="5" covered="0"/>
                <counter type="LINE" missed="2" covered="0"/>
                <counter type="COMPLEXITY" missed="1" covered="0"/>
                <counter type="METHOD" missed="1" covered="0"/>
                <counter type="CLASS" missed="1" covered="0"/>
              </package>
              <package name="dev/mardroemmar/cov/sample">
                <class name="dev/mardroemmar/cov/sample/UncalledCompanionObject$Companion" sourcefilename="UncalledCompanionObject.kt">
                  <method name="neverCalled" desc="()V" line="6">
                    <counter type="INSTRUCTION" missed="5" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="5" covered="0"/>
                  <counter type="LINE" missed="2" covered="0"/>
                  <counter type="COMPLEXITY" missed="1" covered="0"/>
                  <counter type="METHOD" missed="1" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <class name="dev/mardroemmar/cov/sample/UncalledCompanionObject" sourcefilename="UncalledCompanionObject.kt">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="3" covered="0"/>
                    <counter type="LINE" missed="1" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="3" covered="0"/>
                  <counter type="LINE" missed="1" covered="0"/>
                  <counter type="COMPLEXITY" missed="1" covered="0"/>
                  <counter type="METHOD" missed="1" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <class name="dev/mardroemmar/cov/sample/Sample" sourcefilename="Sample.kt">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="0" covered="3"/>
                    <counter type="LINE" missed="0" covered="1"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <method name="neverCalled" desc="()V" line="5">
                    <counter type="INSTRUCTION" missed="5" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <method name="calledOnce" desc="()V" line="9">
                    <counter type="INSTRUCTION" missed="0" covered="5"/>
                    <counter type="LINE" missed="0" covered="2"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <method name="calledManyTimes" desc="(Ljava/lang/String;)V" line="17">
                    <counter type="INSTRUCTION" missed="0" covered="22"/>
                    <counter type="LINE" missed="0" covered="4"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <method name="looped" desc="(Ljava/lang/String;)V" line="23">
                    <counter type="INSTRUCTION" missed="0" covered="18"/>
                    <counter type="LINE" missed="0" covered="3"/>
                    <counter type="COMPLEXITY" missed="0" covered="1"/>
                    <counter type="METHOD" missed="0" covered="1"/>
                  </method>
                  <method name="has a fun snowman! ☃️ 💛" desc="()V" line="28">
                    <counter type="INSTRUCTION" missed="5" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="10" covered="48"/>
                  <counter type="LINE" missed="4" covered="10"/>
                  <counter type="COMPLEXITY" missed="2" covered="4"/>
                  <counter type="METHOD" missed="2" covered="4"/>
                  <counter type="CLASS" missed="0" covered="1"/>
                </class>
                <class name="dev/mardroemmar/cov/sample/UncalledObject" sourcefilename="UncalledObject.kt">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="3" covered="0"/>
                    <counter type="LINE" missed="1" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <method name="neverCalled" desc="()V" line="5">
                    <counter type="INSTRUCTION" missed="5" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="8" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <class name="dev/mardroemmar/cov/sample/UncalledClass" sourcefilename="UncalledClass.kt">
                  <method name="&lt;init&gt;" desc="()V" line="3">
                    <counter type="INSTRUCTION" missed="3" covered="0"/>
                    <counter type="LINE" missed="1" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <method name="neverCalled" desc="()V" line="5">
                    <counter type="INSTRUCTION" missed="5" covered="0"/>
                    <counter type="LINE" missed="2" covered="0"/>
                    <counter type="COMPLEXITY" missed="1" covered="0"/>
                    <counter type="METHOD" missed="1" covered="0"/>
                  </method>
                  <counter type="INSTRUCTION" missed="8" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </class>
                <sourcefile name="Sample.kt">
                  <line nr="3" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="5" mi="4" ci="0" mb="0" cb="0"/>
                  <line nr="6" mi="1" ci="0" mb="0" cb="0"/>
                  <line nr="9" mi="0" ci="4" mb="0" cb="0"/>
                  <line nr="10" mi="0" ci="1" mb="0" cb="0"/>
                  <line nr="17" mi="0" ci="11" mb="0" cb="0"/>
                  <line nr="18" mi="0" ci="4" mb="0" cb="0"/>
                  <line nr="19" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="20" mi="0" ci="1" mb="0" cb="0"/>
                  <line nr="23" mi="0" ci="11" mb="0" cb="0"/>
                  <line nr="24" mi="0" ci="3" mb="0" cb="0"/>
                  <line nr="25" mi="0" ci="1" mb="0" cb="0"/>
                  <line nr="28" mi="4" ci="0" mb="0" cb="0"/>
                  <line nr="29" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="10" covered="48"/>
                  <counter type="LINE" missed="4" covered="10"/>
                  <counter type="COMPLEXITY" missed="2" covered="4"/>
                  <counter type="METHOD" missed="2" covered="4"/>
                  <counter type="CLASS" missed="0" covered="1"/>
                </sourcefile>
                <sourcefile name="UncalledClass.kt">
                  <line nr="3" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="5" mi="4" ci="0" mb="0" cb="0"/>
                  <line nr="6" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="8" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </sourcefile>
                <sourcefile name="UncalledObject.kt">
                  <line nr="3" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="5" mi="4" ci="0" mb="0" cb="0"/>
                  <line nr="6" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="8" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="1" covered="0"/>
                </sourcefile>
                <sourcefile name="UncalledCompanionObject.kt">
                  <line nr="3" mi="3" ci="0" mb="0" cb="0"/>
                  <line nr="6" mi="4" ci="0" mb="0" cb="0"/>
                  <line nr="7" mi="1" ci="0" mb="0" cb="0"/>
                  <counter type="INSTRUCTION" missed="8" covered="0"/>
                  <counter type="LINE" missed="3" covered="0"/>
                  <counter type="COMPLEXITY" missed="2" covered="0"/>
                  <counter type="METHOD" missed="2" covered="0"/>
                  <counter type="CLASS" missed="2" covered="0"/>
                </sourcefile>
                <counter type="INSTRUCTION" missed="34" covered="48"/>
                <counter type="LINE" missed="13" covered="10"/>
                <counter type="COMPLEXITY" missed="8" covered="4"/>
                <counter type="METHOD" missed="8" covered="4"/>
                <counter type="CLASS" missed="4" covered="1"/>
              </package>
              <counter type="INSTRUCTION" missed="42" covered="48"/>
              <counter type="LINE" missed="16" covered="10"/>
              <counter type="COMPLEXITY" missed="10" covered="4"/>
              <counter type="METHOD" missed="10" covered="4"/>
              <counter type="CLASS" missed="6" covered="1"/>
            </report>
"#;
        let report = Report::from_str(report).unwrap();
        assert_eq!(
            report,
            Report {
                name: "sample".into(),
                counters: vec![
                    Counter {
                        typ: CounterType::Instruction,
                        missed: 42,
                        covered: 48
                    },
                    Counter {
                        typ: CounterType::ExecutableLines,
                        missed: 16,
                        covered: 10
                    },
                    Counter {
                        typ: CounterType::Complexity,
                        missed: 10,
                        covered: 4
                    },
                    Counter {
                        typ: CounterType::Method,
                        missed: 10,
                        covered: 4
                    },
                    Counter {
                        typ: CounterType::Class,
                        missed: 6,
                        covered: 1
                    },
                ],
                packages: vec![
                    Package {
                        name: "".into(),
                        classes: vec![Class {
                            name: "Sample".into(),
                            file_name: "Sample.kt".into(),
                            methods: vec![Method {
                                name: "<init>".into(),
                                desc: "()V".into(),
                                line: 2,
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 3,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 1,
                                        covered: 0
                                    },
                                ],
                            }],
                            counters: vec![
                                Counter {
                                    typ: CounterType::Instruction,
                                    missed: 3,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::ExecutableLines,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Complexity,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Method,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Class,
                                    missed: 1,
                                    covered: 0
                                },
                            ],
                        }],
                        counters: vec![
                            Counter {
                                typ: CounterType::Instruction,
                                missed: 3,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::ExecutableLines,
                                missed: 1,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::Complexity,
                                missed: 1,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::Method,
                                missed: 1,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::Class,
                                missed: 1,
                                covered: 0
                            },
                        ],
                        source_files: vec![SourceFile {
                            name: "Sample.kt".into(),
                            lines: vec![Line {
                                number: 2,
                                missed_calls: 3,
                                hit_calls: 0,
                                missed_branches: 0,
                                called_branches: 0,
                            }],
                            counters: vec![
                                Counter {
                                    typ: CounterType::Instruction,
                                    missed: 3,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::ExecutableLines,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Complexity,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Method,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Class,
                                    missed: 1,
                                    covered: 0
                                },
                            ],
                        }],
                    },
                    Package {
                        name: "dev/mardroemmar/cov/sample/uncalled".into(),
                        classes: vec![
                            Class {
                                name: "dev/mardroemmar/cov/sample/uncalled/IAlsoHaveAnObject"
                                    .into(),
                                file_name: "uncalledPackage.kt".into(),
                                methods: vec![],
                                counters: vec![],
                            },
                            Class {
                                name: "dev/mardroemmar/cov/sample/uncalled/UncalledPackageKt"
                                    .into(),
                                file_name: "uncalledPackage.kt".into(),
                                methods: vec![Method {
                                    name: "neverCalled".into(),
                                    desc: "()V".into(),
                                    line: 4,
                                    counters: vec![
                                        Counter {
                                            typ: CounterType::Instruction,
                                            missed: 5,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::ExecutableLines,
                                            missed: 2,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::Complexity,
                                            missed: 1,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::Method,
                                            missed: 1,
                                            covered: 0
                                        },
                                    ],
                                }],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 5,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 2,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 1,
                                        covered: 0
                                    },
                                ],
                            },
                        ],
                        counters: vec![
                            Counter {
                                typ: CounterType::Instruction,
                                missed: 5,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::ExecutableLines,
                                missed: 2,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::Complexity,
                                missed: 1,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::Method,
                                missed: 1,
                                covered: 0
                            },
                            Counter {
                                typ: CounterType::Class,
                                missed: 1,
                                covered: 0
                            },
                        ],
                        source_files: vec![SourceFile {
                            name: "uncalledPackage.kt".into(),
                            lines: vec![
                                Line {
                                    number: 4,
                                    missed_calls: 4,
                                    hit_calls: 0,
                                    missed_branches: 0,
                                    called_branches: 0,
                                },
                                Line {
                                    number: 5,
                                    missed_calls: 1,
                                    hit_calls: 0,
                                    missed_branches: 0,
                                    called_branches: 0,
                                },
                            ],
                            counters: vec![
                                Counter {
                                    typ: CounterType::Instruction,
                                    missed: 5,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::ExecutableLines,
                                    missed: 2,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Complexity,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Method,
                                    missed: 1,
                                    covered: 0
                                },
                                Counter {
                                    typ: CounterType::Class,
                                    missed: 1,
                                    covered: 0
                                },
                            ],
                        }],
                    },
                    Package {
                        name: "dev/mardroemmar/cov/sample".into(),
                        classes: vec![
                            Class {
                                name:
                                    "dev/mardroemmar/cov/sample/UncalledCompanionObject$Companion"
                                        .into(),
                                file_name: "UncalledCompanionObject.kt".into(),
                                methods: vec![Method {
                                    name: "neverCalled".into(),
                                    desc: "()V".into(),
                                    line: 6,
                                    counters: vec![
                                        Counter {
                                            typ: CounterType::Instruction,
                                            missed: 5,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::ExecutableLines,
                                            missed: 2,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::Complexity,
                                            missed: 1,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::Method,
                                            missed: 1,
                                            covered: 0
                                        },
                                    ],
                                }],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 5,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 2,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 1,
                                        covered: 0
                                    },
                                ],
                            },
                            Class {
                                name: "dev/mardroemmar/cov/sample/UncalledCompanionObject".into(),
                                file_name: "UncalledCompanionObject.kt".into(),
                                methods: vec![Method {
                                    name: "<init>".into(),
                                    desc: "()V".into(),
                                    line: 3,
                                    counters: vec![
                                        Counter {
                                            typ: CounterType::Instruction,
                                            missed: 3,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::ExecutableLines,
                                            missed: 1,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::Complexity,
                                            missed: 1,
                                            covered: 0
                                        },
                                        Counter {
                                            typ: CounterType::Method,
                                            missed: 1,
                                            covered: 0
                                        },
                                    ],
                                }],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 3,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 1,
                                        covered: 0
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 1,
                                        covered: 0
                                    },
                                ],
                            },
                            Class {
                                name: "dev/mardroemmar/cov/sample/Sample".into(),
                                file_name: "Sample.kt".into(),
                                methods: vec![
                                    Method {
                                        name: "<init>".into(),
                                        desc: "()V".into(),
                                        line: 3,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 3
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 1
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 0,
                                                covered: 1
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 0,
                                                covered: 1
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "neverCalled".into(),
                                        desc: "()V".into(),
                                        line: 5,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 5,
                                                covered: 0
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 2,
                                                covered: 0
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 1,
                                                covered: 0
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 1,
                                                covered: 0
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "calledOnce".into(),
                                        desc: "()V".into(),
                                        line: 9,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 5
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 2
                                            },
                                            Counter {
                                                typ: CounterType::Complexity,
                                                missed: 0,
                                                covered: 1
                                            },
                                            Counter {
                                                typ: CounterType::Method,
                                                missed: 0,
                                                covered: 1
                                            },
                                        ],
                                    },
                                    Method {
                                        name: "calledManyTimes".into(),
                                        desc: "(Ljava/lang/String;)V".into(),
                                        line: 17,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 22,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 4,
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
                                        name: "looped".into(),
                                        desc: "(Ljava/lang/String;)V".into(),
                                        line: 23,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 0,
                                                covered: 18,
                                            },
                                            Counter {
                                                typ: CounterType::ExecutableLines,
                                                missed: 0,
                                                covered: 3,
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
                                        name: "has a fun snowman! ☃️ 💛".into(),
                                        desc: "()V".into(),
                                        line: 28,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 5,
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
                                        missed: 10,
                                        covered: 48,
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 4,
                                        covered: 10,
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 2,
                                        covered: 4,
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 2,
                                        covered: 4,
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 0,
                                        covered: 1,
                                    },
                                ],
                            },
                            Class {
                                name: "dev/mardroemmar/cov/sample/UncalledObject".into(),
                                file_name: "UncalledObject.kt".into(),
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
                                        line: 5,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 5,
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
                                        missed: 8,
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
                            Class {
                                name: "dev/mardroemmar/cov/sample/UncalledClass".into(),
                                file_name: "UncalledClass.kt".into(),
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
                                        line: 5,
                                        counters: vec![
                                            Counter {
                                                typ: CounterType::Instruction,
                                                missed: 5,
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
                                        missed: 8,
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
                        counters: vec![
                            Counter {
                                typ: CounterType::Instruction,
                                missed: 34,
                                covered: 48,
                            },
                            Counter {
                                typ: CounterType::ExecutableLines,
                                missed: 13,
                                covered: 10,
                            },
                            Counter {
                                typ: CounterType::Complexity,
                                missed: 8,
                                covered: 4,
                            },
                            Counter {
                                typ: CounterType::Method,
                                missed: 8,
                                covered: 4,
                            },
                            Counter {
                                typ: CounterType::Class,
                                missed: 4,
                                covered: 1,
                            },
                        ],
                        source_files: vec![
                            SourceFile {
                                name: "Sample.kt".into(),
                                lines: vec![
                                    Line {
                                        number: 3,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 5,
                                        missed_calls: 4,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 6,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 9,
                                        missed_calls: 0,
                                        hit_calls: 4,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 10,
                                        missed_calls: 0,
                                        hit_calls: 1,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 17,
                                        missed_calls: 0,
                                        hit_calls: 11,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 18,
                                        missed_calls: 0,
                                        hit_calls: 4,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 19,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 20,
                                        missed_calls: 0,
                                        hit_calls: 1,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 23,
                                        missed_calls: 0,
                                        hit_calls: 11,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 24,
                                        missed_calls: 0,
                                        hit_calls: 3,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 25,
                                        missed_calls: 0,
                                        hit_calls: 1,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 28,
                                        missed_calls: 4,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 29,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 10,
                                        covered: 48,
                                    },
                                    Counter {
                                        typ: CounterType::ExecutableLines,
                                        missed: 4,
                                        covered: 10,
                                    },
                                    Counter {
                                        typ: CounterType::Complexity,
                                        missed: 2,
                                        covered: 4,
                                    },
                                    Counter {
                                        typ: CounterType::Method,
                                        missed: 2,
                                        covered: 4,
                                    },
                                    Counter {
                                        typ: CounterType::Class,
                                        missed: 0,
                                        covered: 1,
                                    },
                                ],
                            },
                            SourceFile {
                                name: "UncalledClass.kt".into(),
                                lines: vec![
                                    Line {
                                        number: 3,
                                        missed_calls: 3,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 5,
                                        missed_calls: 4,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 6,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 8,
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
                            SourceFile {
                                name: "UncalledObject.kt".into(),
                                lines: vec![
                                    Line {
                                        number: 3,
                                        missed_calls: 3,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 5,
                                        missed_calls: 4,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 6,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 8,
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
                            SourceFile {
                                name: "UncalledCompanionObject.kt".into(),
                                lines: vec![
                                    Line {
                                        number: 3,
                                        missed_calls: 3,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 6,
                                        missed_calls: 4,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                    Line {
                                        number: 7,
                                        missed_calls: 1,
                                        hit_calls: 0,
                                        missed_branches: 0,
                                        called_branches: 0,
                                    },
                                ],
                                counters: vec![
                                    Counter {
                                        typ: CounterType::Instruction,
                                        missed: 8,
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
                                        missed: 2,
                                        covered: 0,
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        );
    }
}
