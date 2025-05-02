package jacoco_test

import (
	"testing"

	"github.com/proximyst/cov/pkg/report/jacoco"
	"github.com/stretchr/testify/require"
)

func TestParseReport(t *testing.T) {
	t.Parallel()

	cases := map[string]struct {
		input    string
		expected jacoco.Report
	}{
		"valid report": {
			input: `
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
            </report>`,
			expected: jacoco.Report{
				Name: "sample",
				Counters: []jacoco.Counter{
					{Type: jacoco.CounterTypeInstruction, Missed: 18, Covered: 26},
					{Type: jacoco.CounterTypeLine, Missed: 8, Covered: 8},
					{Type: jacoco.CounterTypeComplexity, Missed: 5, Covered: 3},
					{Type: jacoco.CounterTypeMethod, Missed: 5, Covered: 3},
					{Type: jacoco.CounterTypeClass, Missed: 2, Covered: 1},
				},
				Packages: []jacoco.Package{
					{
						Name: "dev/mardroemmar/cov/sample/uncalled",
						Counters: []jacoco.Counter{
							{Type: jacoco.CounterTypeInstruction, Missed: 7, Covered: 0},
							{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
							{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
							{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
							{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
						},
						Classes: []jacoco.Class{
							{
								Name:     "dev/mardroemmar/cov/sample/uncalled/UncalledPackage",
								FileName: "UncalledPackage.java",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 6,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 4, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 7, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
						SourceFiles: []jacoco.SourceFile{
							{
								Name: "UncalledPackage.java",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 7, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 7, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
					},
					{
						Name: "dev/mardroemmar/cov/sample",
						Counters: []jacoco.Counter{
							{Type: jacoco.CounterTypeInstruction, Missed: 11, Covered: 26},
							{Type: jacoco.CounterTypeLine, Missed: 5, Covered: 8},
							{Type: jacoco.CounterTypeComplexity, Missed: 3, Covered: 3},
							{Type: jacoco.CounterTypeMethod, Missed: 3, Covered: 3},
							{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 1},
						},
						Classes: []jacoco.Class{
							{
								Name:     "dev/mardroemmar/cov/sample/Sample",
								FileName: "Sample.java",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 3},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 6,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 4, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
									{
										Name: "calledOnce",
										Desc: "()V",
										Line: 10,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 4},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 2},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
									{
										Name: "calledManyTimes",
										Desc: "(Ljava/lang/String;)V",
										Line: 14,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 19},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 5},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 4, Covered: 26},
									{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 8},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 3},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 3},
									{Type: jacoco.CounterTypeClass, Missed: 0, Covered: 1},
								},
							},
							{
								Name:     "dev/mardroemmar/cov/sample/UncalledClass",
								FileName: "UncalledClass.java",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 6,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 4, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 7, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
						SourceFiles: []jacoco.SourceFile{
							{
								Name: "Sample.java",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 7, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 10, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 11, MissedCalls: 0, HitCalls: 1, MissedBranches: 0, CalledBranches: 0},
									{Number: 14, MissedCalls: 0, HitCalls: 9, MissedBranches: 0, CalledBranches: 0},
									{Number: 15, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 16, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 17, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 18, MissedCalls: 0, HitCalls: 1, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 4, Covered: 26},
									{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 8},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 3},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 3},
									{Type: jacoco.CounterTypeClass, Missed: 0, Covered: 1},
								},
							},
							{
								Name: "UncalledClass.java",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 7, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 7, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
					},
				},
			},
		},
		"kotlin report": {
			input: `
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
`,
			expected: jacoco.Report{
				Name: "sample",
				Counters: []jacoco.Counter{
					{Type: jacoco.CounterTypeInstruction, Missed: 42, Covered: 48},
					{Type: jacoco.CounterTypeLine, Missed: 16, Covered: 10},
					{Type: jacoco.CounterTypeComplexity, Missed: 10, Covered: 4},
					{Type: jacoco.CounterTypeMethod, Missed: 10, Covered: 4},
					{Type: jacoco.CounterTypeClass, Missed: 6, Covered: 1},
				},
				Packages: []jacoco.Package{
					{
						Name: "",
						Counters: []jacoco.Counter{
							{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
							{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
							{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
							{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
							{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
						},
						Classes: []jacoco.Class{
							{
								Name:     "Sample",
								FileName: "Sample.kt",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 2,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
						SourceFiles: []jacoco.SourceFile{
							{
								Name: "Sample.kt",
								Lines: []jacoco.Line{
									{Number: 2, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
					},
					{
						Name: "dev/mardroemmar/cov/sample/uncalled",
						Counters: []jacoco.Counter{
							{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
							{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
							{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
							{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
							{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
						},
						Classes: []jacoco.Class{
							{
								Name:     "dev/mardroemmar/cov/sample/uncalled/IAlsoHaveAnObject",
								FileName: "uncalledPackage.kt",
							},
							{
								Name:     "dev/mardroemmar/cov/sample/uncalled/UncalledPackageKt",
								FileName: "uncalledPackage.kt",
								Methods: []jacoco.Method{
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 4,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
						SourceFiles: []jacoco.SourceFile{
							{
								Name: "uncalledPackage.kt",
								Lines: []jacoco.Line{
									{Number: 4, MissedCalls: 4, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 5, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
					},
					{
						Name: "dev/mardroemmar/cov/sample", Counters: []jacoco.Counter{
							{Type: jacoco.CounterTypeInstruction, Missed: 34, Covered: 48},
							{Type: jacoco.CounterTypeLine, Missed: 13, Covered: 10},
							{Type: jacoco.CounterTypeComplexity, Missed: 8, Covered: 4},
							{Type: jacoco.CounterTypeMethod, Missed: 8, Covered: 4},
							{Type: jacoco.CounterTypeClass, Missed: 4, Covered: 1},
						},
						Classes: []jacoco.Class{
							{
								Name:     "dev/mardroemmar/cov/sample/UncalledCompanionObject$Companion",
								FileName: "UncalledCompanionObject.kt",
								Methods: []jacoco.Method{
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 6,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
							{
								Name:     "dev/mardroemmar/cov/sample/UncalledCompanionObject",
								FileName: "UncalledCompanionObject.kt",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
							{
								Name:     "dev/mardroemmar/cov/sample/Sample",
								FileName: "Sample.kt",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 3},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 5,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
									{
										Name: "calledOnce",
										Desc: "()V",
										Line: 9,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 5},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 2},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
									{
										Name: "calledManyTimes",
										Desc: "(Ljava/lang/String;)V",
										Line: 17,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 22},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 4},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
									{
										Name: "looped",
										Desc: "(Ljava/lang/String;)V",
										Line: 23,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 0, Covered: 18},
											{Type: jacoco.CounterTypeLine, Missed: 0, Covered: 3},
											{Type: jacoco.CounterTypeComplexity, Missed: 0, Covered: 1},
											{Type: jacoco.CounterTypeMethod, Missed: 0, Covered: 1},
										},
									},
									{
										Name: "has a fun snowman! ☃️ 💛",
										Desc: "()V",
										Line: 28,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 10, Covered: 48},
									{Type: jacoco.CounterTypeLine, Missed: 4, Covered: 10},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 4},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 4},
									{Type: jacoco.CounterTypeClass, Missed: 0, Covered: 1},
								},
							},
							{
								Name:     "dev/mardroemmar/cov/sample/UncalledObject",
								FileName: "UncalledObject.kt",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 5,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 8, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
							{
								Name:     "dev/mardroemmar/cov/sample/UncalledClass",
								FileName: "UncalledClass.kt",
								Methods: []jacoco.Method{
									{
										Name: "<init>",
										Desc: "()V",
										Line: 3,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 3, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
									{
										Name: "neverCalled",
										Desc: "()V",
										Line: 5,
										Counters: []jacoco.Counter{
											{Type: jacoco.CounterTypeInstruction, Missed: 5, Covered: 0},
											{Type: jacoco.CounterTypeLine, Missed: 2, Covered: 0},
											{Type: jacoco.CounterTypeComplexity, Missed: 1, Covered: 0},
											{Type: jacoco.CounterTypeMethod, Missed: 1, Covered: 0},
										},
									},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 8, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
						},
						SourceFiles: []jacoco.SourceFile{
							{
								Name: "Sample.kt",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 5, MissedCalls: 4, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 9, MissedCalls: 0, HitCalls: 4, MissedBranches: 0, CalledBranches: 0},
									{Number: 10, MissedCalls: 0, HitCalls: 1, MissedBranches: 0, CalledBranches: 0},
									{Number: 17, MissedCalls: 0, HitCalls: 11, MissedBranches: 0, CalledBranches: 0},
									{Number: 18, MissedCalls: 0, HitCalls: 4, MissedBranches: 0, CalledBranches: 0},
									{Number: 19, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 20, MissedCalls: 0, HitCalls: 1, MissedBranches: 0, CalledBranches: 0},
									{Number: 23, MissedCalls: 0, HitCalls: 11, MissedBranches: 0, CalledBranches: 0},
									{Number: 24, MissedCalls: 0, HitCalls: 3, MissedBranches: 0, CalledBranches: 0},
									{Number: 25, MissedCalls: 0, HitCalls: 1, MissedBranches: 0, CalledBranches: 0},
									{Number: 28, MissedCalls: 4, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 29, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 10, Covered: 48},
									{Type: jacoco.CounterTypeLine, Missed: 4, Covered: 10},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 4},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 4},
									{Type: jacoco.CounterTypeClass, Missed: 0, Covered: 1},
								},
							},
							{
								Name: "UncalledClass.kt",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 5, MissedCalls: 4, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 8, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
							{
								Name: "UncalledObject.kt",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 5, MissedCalls: 4, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 8, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 1, Covered: 0},
								},
							},
							{
								Name: "UncalledCompanionObject.kt",
								Lines: []jacoco.Line{
									{Number: 3, MissedCalls: 3, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 6, MissedCalls: 4, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
									{Number: 7, MissedCalls: 1, HitCalls: 0, MissedBranches: 0, CalledBranches: 0},
								},
								Counters: []jacoco.Counter{
									{Type: jacoco.CounterTypeInstruction, Missed: 8, Covered: 0},
									{Type: jacoco.CounterTypeLine, Missed: 3, Covered: 0},
									{Type: jacoco.CounterTypeComplexity, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeMethod, Missed: 2, Covered: 0},
									{Type: jacoco.CounterTypeClass, Missed: 2, Covered: 0},
								},
							},
						},
					},
				},
			},
		},
	}
	for name, tc := range cases {
		t.Run(name, func(t *testing.T) {
			t.Parallel()

			report, err := jacoco.ParseReport([]byte(tc.input))
			require.NoError(t, err, "failed to parse report")
			require.Equal(t, tc.expected, report, "parsed report should match expected")
		})
	}
}
