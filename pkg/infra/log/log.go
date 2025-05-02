package log

import (
	"errors"
	"log/slog"
	"os"
	"time"

	"github.com/lmittmann/tint"
)

var ErrLogLevelUnknown = errors.New("unknown log level (must be one of: debug, info, warn, error)")

// SetupLogger initializes the logger with the specified log level and output format.
// See `CreateLogger` for more details.
func SetupLogger(logLevel string, writeJSON bool) (*slog.Logger, error) {
	logger, err := CreateLogger(logLevel, writeJSON)
	if err != nil {
		return nil, err
	}
	slog.SetDefault(logger)
	return logger, nil
}

// CreateLogger creates a new logger with the specified log level and output format.
// If `writeJSON` is true, the logger will output JSON formatted logs, otherwise a human-readable format will be used.
// The log level can be one of "debug", "info", "warn", or "error".
// If an unknown log level is provided, an error will be returned.
// The logger will include source information (file and line number) in the output.
func CreateLogger(logLevel string, writeJSON bool) (*slog.Logger, error) {
	var level slog.Level
	switch logLevel {
	case "debug":
		level = slog.LevelDebug
	case "info":
		level = slog.LevelInfo
	case "warn":
		level = slog.LevelWarn
	case "error":
		level = slog.LevelError
	default:
		return nil, ErrLogLevelUnknown
	}

	var handler slog.Handler
	if writeJSON {
		handler = slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
			Level:     level,
			AddSource: true,
		})
	} else {
		handler = tint.NewHandler(os.Stdout, &tint.Options{
			AddSource:  true,
			Level:      level,
			TimeFormat: time.RFC3339Nano,
		})
	}

	return slog.New(handler), nil
}
