package cmd

import (
	"context"

	"github.com/proximyst/cov/cmd/admin"
	"github.com/proximyst/cov/cmd/migrate"
	"github.com/proximyst/cov/cmd/server"
	"github.com/proximyst/cov/pkg/infra/log"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:  "cov",
		Usage: "cov is a code coverage service.",
		Description: `cov is a code coverage service that lets you track your code coverage over time with a simple web interface.

All options can also be set via environment variables. The environment variable names are prefixed with COV_ and the dashes are replaced with underscores. For example, --log-level=debug becomes COV_LOG_LEVEL=debug.`,
		Before: func(ctx context.Context, c *cli.Command) (context.Context, error) {
			if err := log.SetupLoggerFromCommand(c); err != nil {
				return ctx, err
			}
			return ctx, nil
		},
		Flags: []cli.Flag{
			log.FlagLogLevel(),
			log.FlagLogJSON(),
		},
		Commands: []*cli.Command{
			admin.New(),
			migrate.New(),
			server.New(),
		},
	}
}
