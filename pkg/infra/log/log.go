package log

import (
	"errors"
	"log/slog"
	"os"
	"time"

	"github.com/lmittmann/tint"
)

type LogFlags struct {
	Level  string `help:"Set the log level (${enum})" enum:"debug, info, warn, error" default:"info"`
	Format string `help:"The format to output logs in (${enum})" default:"colour" enum:"plain, colour, ndjson"`
}

func (f LogFlags) AfterApply() error {
	logger, err := f.createLogger()
	if err != nil {
		return err
	}
	slog.SetDefault(logger)
	return nil
}

func (f LogFlags) createLogger() (*slog.Logger, error) {
	var level slog.Level
	switch f.Level {
	case "debug":
		level = slog.LevelDebug
	case "info":
		level = slog.LevelInfo
	case "warn":
		level = slog.LevelWarn
	case "error":
		level = slog.LevelError
	default:
		return nil, errors.New("unknown log level") // unreachable due to enum validation
	}

	writer := os.Stdout

	var handler slog.Handler
	switch f.Format {
	case "ndjson":
		handler = slog.NewJSONHandler(writer, &slog.HandlerOptions{
			Level:     level,
			AddSource: true,
		})
	case "plain":
		handler = slog.NewTextHandler(writer, &slog.HandlerOptions{
			Level:     level,
			AddSource: true,
		})
	case "colour":
		handler = tint.NewHandler(writer, &tint.Options{
			AddSource:  true,
			Level:      level,
			TimeFormat: time.RFC3339Nano,
		})
	default:
		return nil, errors.New("unknown log format") // unreachable due to enum validation
	}

	return slog.New(handler), nil
}
