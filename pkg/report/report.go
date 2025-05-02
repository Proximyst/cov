package report

import (
	"errors"

	"github.com/proximyst/cov/pkg/report/golang"
	"github.com/proximyst/cov/pkg/report/jacoco"
)

// An aggregate report of all covered files uploaded.
type Report struct {
	// RawReport is the underlying tool-specific report.
	// This is unlikely to be particularly useful, as the aggregate, generic report should have the same data.
	RawReport any `json:"-"`

	// Regions contains all the code regions in the report.
	Regions []Region `json:"regions"`
}

// Region represents a code region.
// This can be a partial line, a full line, partial multiple lines, or full multiple lines.
type Region struct {
	// File is the file path of the code region.
	// This can be either a relative path (usually from the root of the repository) or an absolute path (can include user home and similar).
	File string `json:"file"`

	// FromLine is the starting line of the code region.
	// This is 1-indexed, meaning the first line of the file is line 1.
	//
	// It is always true that `FromLine <= ToLine`.
	// It is always true that `FromLine >= 1`.
	FromLine int `json:"from_line"`

	// ToLine is the ending line of the code region.
	// This is inclusive, meaning the region includes the line at ToLine.
	// This is 1-indexed, meaning the first line of the file is line 1.
	//
	// It is always true that `FromLine <= ToLine`.
	ToLine int `json:"to_line"`

	// FromColumn is the starting column of the code region.
	// This is 0-indexed, meaning the first column of the line is column 0.
	FromColumn int `json:"from_column"`

	// ToColumn is the ending column of the code region.
	// This is inclusive, meaning the region includes the column at ToColumn.
	// This is 0-indexed, meaning the first column of the line is column 0.
	// If this is 0, it means the region ends at the end of the previous line. This is an optimisation to avoid reading the file to find the end column of said line.
	//
	// It is always true that `FromColumn <= ToColumn`.
	ToColumn int `json:"to_column"`

	// Statements is the number of statements in the code region.
	// The idea of a statement can vary between languages, e.g. individual function calls or JVM byte code instructions.
	//
	// This is not limited by `Executions`.
	Statements int `json:"statements"`

	// Executions is the number of times the code region was executed.
	// A single region can be executed multiple times, e.g. in a loop, and as such this count can be very large.
	//
	// This is not limited by `Statements`.
	Executions int `json:"executions"`
}

var ErrInvalidReport = errors.New("invalid report")

func Parse(report []byte) (Report, error) {
	golangReport, err := golang.ParseReport(report)
	if err == nil {
		return fromGolang(golangReport), nil
	}

	jacocoReport, err := jacoco.ParseReport(report)
	if err == nil {
		return fromJacoco(jacocoReport), nil
	}

	return Report{}, ErrInvalidReport
}

func fromGolang(report golang.Report) Report {
	regions := make([]Region, len(report.Regions))
	for i, region := range report.Regions {
		regions[i] = Region{
			File:       region.File,
			FromLine:   region.StartLine,
			ToLine:     region.EndLine,
			FromColumn: region.StartColumn,
			ToColumn:   region.EndColumn,
			Statements: region.Statements,
			Executions: region.Executed,
		}
	}

	return Report{
		RawReport: report,
		Regions:   regions,
	}
}

func fromJacoco(report jacoco.Report) Report {
	var regions []Region
	for _, pkg := range report.Packages {
		for _, file := range pkg.SourceFiles {
			for _, line := range file.Lines {
				regions = append(regions, Region{
					File:       file.Name,
					FromLine:   line.Number,
					FromColumn: 0,
					ToLine:     line.Number + 1,
					ToColumn:   0,
					Statements: line.HitCalls + line.MissedCalls,
					Executions: line.HitCalls,
				})
			}
		}
	}

	return Report{
		RawReport: report,
		Regions:   regions,
	}
}
