package log

import (
	"github.com/proximyst/cov/pkg/infra/env"
	"github.com/urfave/cli/v3"
)

func FlagLogLevel() cli.Flag {
	return &cli.StringFlag{
		Name:  "log-level",
		Usage: "Set the log level. (enum: debug, info, warn, error)",
		Value: env.Get("LOG_LEVEL", "info"),
		Validator: func(value string) error {
			switch value {
			case "debug", "info", "warn", "error":
				return nil
			default:
				return cli.Exit("unknown log level (enum: debug, info, warn, error)", 1)
			}
		},
		Category: "Logging",
	}
}

func FlagLogJSON() cli.Flag {
	return &cli.BoolFlag{
		Name:     "log-json",
		Usage:    "Output logs in JSON format.",
		Value:    env.GetBool("LOG_JSON", false),
		Category: "Logging",
	}
}

func SetupLoggerFromCommand(c *cli.Command) error {
	logLevel := c.String("log-level")
	writeJSON := c.Bool("log-json")
	_, err := SetupLogger(logLevel, writeJSON)
	if err != nil {
		return cli.Exit("failed to setup logger: "+err.Error(), 1)
	}
	return nil
}
