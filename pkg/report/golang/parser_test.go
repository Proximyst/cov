package golang_test

import (
	"testing"

	"github.com/proximyst/cov/pkg/report/golang"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestParseReport(t *testing.T) {
	t.Parallel()

	t.Run("parse report with no regions", func(t *testing.T) {
		t.Parallel()

		for _, mode := range []golang.ReportMode{golang.ModeSet, golang.ModeCount, golang.ModeAtomic} {
			t.Run("with mode: "+mode.String(), func(t *testing.T) {
				t.Parallel()

				text := "mode: " + mode.String() + "\n"
				report, err := golang.ParseReport([]byte(text))
				require.NoError(t, err, "failed to parse report")
				assert.Equal(t, mode, report.Mode, "mode should be set to %s", mode)
				assert.Empty(t, report.Regions, "regions should be empty")
			})
		}

		t.Run("with no mode line", func(t *testing.T) {
			t.Parallel()

			text := "\n"
			_, err := golang.ParseReport([]byte(text))
			var invalidReport *golang.ErrInvalidReport
			require.ErrorAs(t, err, &invalidReport, "should return an invalid report error")
		})

		t.Run("with no mode value", func(t *testing.T) {
			t.Parallel()

			text := "mode: \n"
			_, err := golang.ParseReport([]byte(text))
			var invalidReport *golang.ErrInvalidReport
			require.ErrorAs(t, err, &invalidReport, "should return an invalid report error")
		})

		t.Run("with invalid mode", func(t *testing.T) {
			t.Parallel()

			text := "mode: invalid\n"
			_, err := golang.ParseReport([]byte(text))
			var invalidReport *golang.ErrInvalidReport
			require.ErrorAs(t, err, &invalidReport, "should return an invalid report error")
		})
	})

	t.Run("parse report with one region", func(t *testing.T) {
		t.Parallel()

		text := `mode: set
github.com/owner/repo/file.go:1.2,3.4 5 6`
		report, err := golang.ParseReport([]byte(text))
		require.NoError(t, err, "failed to parse report")
		assert.Equal(t, golang.ModeSet, report.Mode, "mode should be set to set")
		require.Len(t, report.Regions, 1, "should have one region")

		region := report.Regions[0]
		assert.Equal(t, "github.com/owner/repo/file.go", region.File, "file path should be set")
		assert.Equal(t, 1, region.StartLine, "start line should be set to 1")
		assert.Equal(t, 2, region.StartColumn, "start column should be set to 2")
		assert.Equal(t, 3, region.EndLine, "end line should be set to 3")
		assert.Equal(t, 4, region.EndColumn, "end column should be set to 4")
		assert.Equal(t, 5, region.Statements, "statements should be set to 5")
		assert.Equal(t, 6, region.Executed, "executed should be set to 6")
	})

	t.Run("parse report with multiple regions", func(t *testing.T) {
		t.Parallel()

		// Yes, the whitespace is intentional.
		text := `mode: count

github.com/owner/repo/file.go:1.2,3.4 5 6
github.com/owner/repo/file.go:7.8,9.10 11 12

`
		report, err := golang.ParseReport([]byte(text))
		require.NoError(t, err, "failed to parse report")
		assert.Equal(t, golang.ModeCount, report.Mode, "mode should be set to count")
		require.Len(t, report.Regions, 2, "should have two regions")

		region := report.Regions[0]
		assert.Equal(t, "github.com/owner/repo/file.go", region.File, "file path should be set")
		assert.Equal(t, 1, region.StartLine, "start line should be set to 1")
		assert.Equal(t, 2, region.StartColumn, "start column should be set to 2")
		assert.Equal(t, 3, region.EndLine, "end line should be set to 3")
		assert.Equal(t, 4, region.EndColumn, "end column should be set to 4")
		assert.Equal(t, 5, region.Statements, "statements should be set to 5")
		assert.Equal(t, 6, region.Executed, "executed should be set to 6")

		region = report.Regions[1]
		assert.Equal(t, "github.com/owner/repo/file.go", region.File, "file path should be set")
		assert.Equal(t, 7, region.StartLine, "start line should be set to 7")
		assert.Equal(t, 8, region.StartColumn, "start column should be set to 8")
		assert.Equal(t, 9, region.EndLine, "end line should be set to 9")
		assert.Equal(t, 10, region.EndColumn, "end column should be set to 10")
		assert.Equal(t, 11, region.Statements, "statements should be set to 11")
		assert.Equal(t, 12, region.Executed, "executed should be set to 12")
	})

	t.Run("another format returns ErrInvalidReport", func(t *testing.T) {
		t.Parallel()

		// This probably isn't a valid report for anything.
		text := `<?xml version="1.0" encoding="UTF-8"?>
<coverage version="1.0">
</coverage>`
		_, err := golang.ParseReport([]byte(text))
		var invalidReport *golang.ErrInvalidReport
		require.ErrorAs(t, err, &invalidReport, "should return an invalid report error")
	})
}
