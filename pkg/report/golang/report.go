package golang

// Report is a Go-specific report.
type Report struct {
	// Mode is the form of counting this Go report uses.
	Mode ReportMode `json:"mode"`
	// Regions is the list of regions in the report.
	Regions []Region `json:"regions"`
}

// ReportMode is an enum of the different modes of reporting.
type ReportMode int

const (
	// ModeUnknown is the default mode, and should not be used.
	// This is used when the mode is not known, or the report is not a valid Go report.
	// It is the zero-value of ReportMode.
	ModeUnknown ReportMode = iota
	// ModeSet answers the question of "Was this line executed at all?".
	ModeSet
	// ModeCount answers the question of "How many times was this line executed?".
	// It is unsafe in concurrent programs.
	ModeCount
	// ModeAtomic answers the question of "How many times was this line executed?".
	// It is safe in concurrent programs.
	ModeAtomic
)

func (m ReportMode) String() string {
	switch m {
	case ModeSet:
		return "set"
	case ModeCount:
		return "count"
	case ModeAtomic:
		return "atomic"
	default:
		return "unknown"
	}
}

func (m ReportMode) IsValid() bool {
	switch m {
	case ModeSet, ModeCount, ModeAtomic:
		return true
	default:
		return false
	}
}

type Region struct {
	// File is the file path of the code region.
	// This is usually a relative path from the root of the repository.
	File string `json:"file"`
	// FromLine is the starting line of the code region.
	// This is 1-indexed, meaning the first line of the file is line 1.
	StartLine int `json:"start_line"`
	// EndLine is the ending line of the code region.
	// This is inclusive, meaning the region includes the line at EndColumn.
	EndLine int `json:"end_line"`
	// StartColumn is the starting column of the code region.
	// This is 0-indexed, meaning the first column of the line is column 0.
	StartColumn int `json:"start_column"`
	// EndColumn is the ending column of the code region.
	// This is inclusive, meaning the region includes the column at EndColumn.
	// It is guaranteed that `StartColumn <= EndColumn`.
	EndColumn int `json:"end_column"`
	// Statements is the number of statements in the code region.
	Statements int `json:"statements"`
	// Executed is the number of times the code region was executed.
	// This is not limited by `Statements`.
	//
	// If Mode is ModeSet, this is 0 or 1.
	// If Mode is ModeCount or ModeAtomic, this is the number of times the region was executed across the runtime of the test suite.
	// In ModeCount, this might be wrong if the code uses parallelism anywhere.
	Executed int `json:"executed"`
}
