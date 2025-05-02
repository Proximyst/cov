package golang

import (
	"errors"
	"io"
	"slices"
	"strconv"

	"github.com/proximyst/cov/pkg/report/ctxerr"
	"github.com/proximyst/cov/pkg/report/cursor"
)

var _ error = (*ErrInvalidReport)(nil)

type ErrInvalidReport struct {
	// err is the error that caused the invalid report. It may be nil.
	err error
	ctx string
}

func (e *ErrInvalidReport) Error() string {
	if e.err == nil && e.ctx == "" {
		return "invalid report"
	} else if e.err == nil {
		return "invalid report (" + e.ctx + ")"
	} else if e.ctx == "" {
		return "invalid report: " + e.err.Error()
	} else {
		return "invalid report (" + e.ctx + "): " + e.err.Error()
	}
}

func (e *ErrInvalidReport) Unwrap() error {
	return e.err
}

// ParseReport parses a Go-specific report from the given reader.
// It returns a Report struct containing the parsed data.
// If the report is not a valid Go report, it returns an error.
func ParseReport(report []byte) (Report, error) {
	c := cursor.Wrap(report)

	mode, err := parseMode(c)
	if err != nil {
		return Report{}, ctxerr.New(err, "parsing mode string")
	}

	// Parse the regions in the report.
	var regions []Region
	for {
		// Mark & reset isn't the most efficient way to do this, but it makes the code easier to read, and deals with \r\n.
		c.Mark()
		line, err := c.ReadLine()
		if errors.Is(err, io.EOF) {
			break
		}
		if err != nil {
			return Report{}, &ErrInvalidReport{err, "reading line"}
		}
		if len(line) == 0 {
			// This is a blank line, so we can skip it.
			// It shouldn't happen, but it's easy to ignore, especially if some tool requires all files end with a new line or similar.
			continue
		}
		c.Reset()
		// This is a line of data.

		region, err := parseRegion(c)
		if err != nil {
			return Report{}, &ErrInvalidReport{err, "parsing region"}
		}
		regions = append(regions, region)
	}

	return Report{
		Mode:    mode,
		Regions: regions,
	}, nil
}

// parseMode reads the `mode: set` / `mode: count` / `mode: atomic` line from the report.
// It does not consume the new line at the end of the line, if any.
func parseMode(c *cursor.C) (ReportMode, error) {
	line, err := c.ReadLine()
	if err != nil {
		return ModeUnknown, ctxerr.New(err, "reading mode line")
	}

	if slices.Equal(line, []byte("mode: set")) {
		return ModeSet, nil
	} else if slices.Equal(line, []byte("mode: count")) {
		return ModeCount, nil
	} else if slices.Equal(line, []byte("mode: atomic")) {
		return ModeAtomic, nil
	}
	return ModeUnknown, &ErrInvalidReport{nil, "mode line is not valid"}
}

func parseRegion(c *cursor.C) (Region, error) {
	// format: "file_path:start_line.start_column,end_line.end_column statements executed"
	// e.g.: "pkg/foo.go:1.2,3.4 5 6"

	filePath, err := c.ReadTill(':')
	if err != nil {
		return Region{}, ctxerr.New(err, "reading file path")
	}

	startLineBytes, err := c.ReadTill('.')
	if err != nil {
		return Region{}, ctxerr.New(err, "reading start line")
	}
	startLine, err := strconv.Atoi(string(startLineBytes))
	if err != nil {
		return Region{}, ctxerr.New(err, "parsing start line")
	}

	startColumnBytes, err := c.ReadTill(',')
	if err != nil {
		return Region{}, ctxerr.New(err, "reading start column")
	}
	startColumn, err := strconv.Atoi(string(startColumnBytes))
	if err != nil {
		return Region{}, ctxerr.New(err, "parsing start column")
	}

	endLineBytes, err := c.ReadTill('.')
	if err != nil {
		return Region{}, ctxerr.New(err, "reading end line")
	}
	endLine, err := strconv.Atoi(string(endLineBytes))
	if err != nil {
		return Region{}, ctxerr.New(err, "parsing end line")
	}

	endColumnBytes, err := c.ReadTill(' ')
	if err != nil {
		return Region{}, ctxerr.New(err, "reading end column")
	}
	endColumn, err := strconv.Atoi(string(endColumnBytes))
	if err != nil {
		return Region{}, ctxerr.New(err, "parsing end column")
	}

	statementsBytes, err := c.ReadTill(' ')
	if err != nil {
		return Region{}, ctxerr.New(err, "reading statements")
	}
	statements, err := strconv.Atoi(string(statementsBytes))
	if err != nil {
		return Region{}, ctxerr.New(err, "parsing statements")
	}

	executedBytes, err := c.ReadLine()
	if err != nil {
		return Region{}, ctxerr.New(err, "reading executed")
	}
	executed, err := strconv.Atoi(string(executedBytes))
	if err != nil {
		return Region{}, ctxerr.New(err, "parsing executed")
	}

	return Region{
		File:        string(filePath),
		StartLine:   startLine,
		StartColumn: startColumn,
		EndLine:     endLine,
		EndColumn:   endColumn,
		Statements:  statements,
		Executed:    executed,
	}, nil
}
