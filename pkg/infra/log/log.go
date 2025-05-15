package log

import (
	"errors"
	"log/slog"
	"os"
	"time"

	"github.com/lmittmann/tint"
	"github.com/proximyst/cov/pkg/infra/closer"
)

type LogFlags struct {
	Level  string `help:"Set the log level (${enum})" enum:"debug, info, warn, error" default:"info"`
	Format string `help:"The format to output logs in (${enum})" default:"colour" enum:"plain, colour, ndjson"`
	File   string `help:"Path to the log file (- being stdout)" default:"-" type:"path"`
}

func (f LogFlags) AfterApply(c *closer.C) error {
	logger, err := f.createLogger(c)
	if err != nil {
		return err
	}
	slog.SetDefault(logger)
	return nil
}

func (f LogFlags) createLogger(c *closer.C) (*slog.Logger, error) {
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
	if f.File != "-" {
		file, err := os.OpenFile(f.File, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		c.Add(file.Close)

		writer = file
	}

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
