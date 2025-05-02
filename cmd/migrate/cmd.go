package migrate

import (
	"context"

	"github.com/proximyst/cov/pkg/db"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:  "migrate",
		Usage: "Run database migrations.",
		Flags: []cli.Flag{
			db.FlagConnectionString(),
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			return run(ctx, c.String("database"))
		},
	}
}
