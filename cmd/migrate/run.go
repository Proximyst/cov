package migrate

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/proximyst/cov/pkg/db"
)

func run(ctx context.Context, connString string) error {
	pool, err := db.Connect(ctx, connString)
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
