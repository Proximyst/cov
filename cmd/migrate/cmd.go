package migrate

import (
	"context"
	"fmt"
	"log/slog"

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
			pool, err := db.ConnectFromCommand(ctx, c)
			if err != nil {
				return fmt.Errorf("failed to connect to database: %w", err)
			}
			defer pool.Close()

			migrations, err := db.EmbeddedMigrationsSource()
			if err != nil {
				return fmt.Errorf("failed to get embedded migrations: %w", err)
			}

			slog.Debug("running migrations on database")
			if err := db.ExecuteMigrations(ctx, migrations, pool); err != nil {
				return fmt.Errorf("failed to run migrations: %w", err)
			}
			slog.Info("migrations completed successfully")

			return nil
		},
	}
}
