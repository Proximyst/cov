package jacoco

import (
	"encoding"
	"encoding/xml"
	"errors"
)

var ErrInvalidReport = errors.New("invalid report")

// Report is a JaCoCo-specific report.
type Report struct {
	// Name is the name of the report.
	// This is usually the same as the artifactId or name of the Maven/Gradle project.
	Name string `xml:"name,attr"`

	Counters []Counter `xml:"counter"`
	Packages []Package `xml:"package"`
}

type Counter struct {
	Type    CounterType `xml:"type,attr"`
	Missed  int         `xml:"missed,attr"`
	Covered int         `xml:"covered,attr"`
}

type CounterType int

const (
	CounterTypeUnknown CounterType = iota
	CounterTypeInstruction
	CounterTypeLine
	CounterTypeComplexity
	CounterTypeMethod
	CounterTypeClass
)

func (c *CounterType) UnmarshalText(text []byte) error {
	switch string(text) {
	case "INSTRUCTION":
		*c = CounterTypeInstruction
	case "LINE":
		*c = CounterTypeLine
	case "COMPLEXITY":
		*c = CounterTypeComplexity
	case "METHOD":
		*c = CounterTypeMethod
	case "CLASS":
		*c = CounterTypeClass
	default:
		return ErrInvalidReport
	}
	return nil
}

func (c CounterType) String() string {
	switch c {
	case CounterTypeInstruction:
		return "INSTRUCTION"
	case CounterTypeLine:
		return "LINE"
	case CounterTypeComplexity:
		return "COMPLEXITY"
	case CounterTypeMethod:
		return "METHOD"
	case CounterTypeClass:
		return "CLASS"
	default:
		return "UNKNOWN"
	}
}

var _ encoding.TextUnmarshaler = (*CounterType)(nil)

type Package struct {
	Name        string       `xml:"name,attr"`
	Counters    []Counter    `xml:"counter"`
	Classes     []Class      `xml:"class"`
	SourceFiles []SourceFile `xml:"sourcefile"`
}

type Class struct {
	Name     string    `xml:"name,attr"`
	FileName string    `xml:"sourcefilename,attr"`
	Methods  []Method  `xml:"method"`
	Counters []Counter `xml:"counter"`
}

type Method struct {
	Name     string    `xml:"name,attr"`
	Desc     string    `xml:"desc,attr"`
	Line     int       `xml:"line,attr"`
	Counters []Counter `xml:"counter"`
}

type SourceFile struct {
	Name     string    `xml:"name,attr"`
	Lines    []Line    `xml:"line"`
	Counters []Counter `xml:"counter"`
}

type Line struct {
	Number         int `xml:"nr,attr"`
	MissedCalls    int `xml:"mi,attr"`
	HitCalls       int `xml:"ci,attr"`
	MissedBranches int `xml:"mb,attr"`
	CalledBranches int `xml:"cb,attr"`
}

func ParseReport(report []byte) (Report, error) {
	var r Report
	if err := xml.Unmarshal(report, &r); err != nil {
		return Report{}, ErrInvalidReport
	}
	return r, nil
}
