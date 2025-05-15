package migrate

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/proximyst/cov/pkg/db"
)

type Command struct {
	Database db.Flags `embed:""`
}

func (c *Command) Run(ctx context.Context) error {
	pool, err := c.Database.Connect(ctx)
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
}
