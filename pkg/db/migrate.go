package db

import (
	"context"
	"embed"
	"errors"
	"fmt"

	"github.com/golang-migrate/migrate/v4"
	pgxdriver "github.com/golang-migrate/migrate/v4/database/pgx/v5"
	"github.com/golang-migrate/migrate/v4/source"
	"github.com/golang-migrate/migrate/v4/source/iofs"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/jackc/pgx/v5/stdlib"
)

//go:embed migrations/*.sql
var migrationsFS embed.FS

// EmbeddedMigrationsSource returns a source driver for the embedded migrations.
// These files are embedded into the binary using the `embed` package.
func EmbeddedMigrationsSource() (source.Driver, error) {
	return iofs.New(migrationsFS, "migrations")
}

// ExecuteMigrations executes the migrations against the provided database connection pool.
// It leaves the database pool open for further use.
// It will return an error if any migration fails, or if the source driver cannot be created.
// It will not return an error if there are no changes to be made (i.e. the database is already up to date).
func ExecuteMigrations(ctx context.Context, migrations source.Driver, db *pgxpool.Pool) error {
	stdlibDB := stdlib.OpenDBFromPool(db)
	defer stdlibDB.Close()
	driver, err := pgxdriver.WithInstance(stdlibDB, &pgxdriver.Config{})
	if err != nil {
		return fmt.Errorf("failed to create pgx driver instance: %w", err)
	}
	defer driver.Close()

	mig, err := migrate.NewWithInstance("migrations", migrations, "database", driver)
	if err != nil {
		return fmt.Errorf("failed to create new migrate instance: %w", err)
	}
	// We don't need to close the migrate instance.

	done := make(chan struct{})
	defer close(done)
	go func() {
		select {
		case <-ctx.Done():
			mig.GracefulStop <- true
		case <-done:
			// This is a no-op. We just want to ensure that the goroutine exits.
		}
	}()

	if err := mig.Up(); err != nil && !errors.Is(err, migrate.ErrNoChange) {
		return fmt.Errorf("failed to run migrations: %w", err)
	}
	return ctx.Err()
}
