package db

import (
	"context"
	"fmt"
	"io"
	"math/rand/v2"
	"os"
	"path/filepath"
	"strings"

	"github.com/golang-migrate/migrate/v4/source"
	_ "github.com/golang-migrate/migrate/v4/source/file" // registers itself with an init function
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/proximyst/cov/pkg/infra/env"
)

type testDatabaseOptions struct {
	// AdditionalMigrations is a map of migration names to their SQL file contents.
	// They are added between the embedded migrations according to the order of the keys.
	// This allows for testing migrations by adding fake data at different points in the migration process.
	additionalMigrations map[string]string
	// DisableMigrations disables the migrations from running at all.
	// This ignores the additional migrations as well.
	disableMigrations bool
}

type testDatabaseOption = func(*testDatabaseOptions)

func WithAdditionalMigration(name, content string) testDatabaseOption {
	return func(opts *testDatabaseOptions) {
		opts.additionalMigrations[name] = content
	}
}

func WithoutMigrations() testDatabaseOption {
	return func(opts *testDatabaseOptions) {
		opts.disableMigrations = true
	}
}

// TestingT is an interface that maps to a subset of the testing.TB interface.
type testingT interface {
	// Push a function to the cleanup stack.
	// The function is called when the test ends.
	// It is called in reverse order of the calls to Cleanup, like a LIFO stack.
	Cleanup(func())

	// Gets a context which lives for the duration of the test.
	Context() context.Context

	// Mark the calling function as a test helper.
	Helper()

	// Logf formats its arguments according to the format, and logs them.
	Logf(format string, args ...any)

	// TempDir creates a new temporary directory for this test.
	// It is cleaned up by the testing framework when the test ends.
	TempDir() string
}

// CreateTestDB connects to the running test database and returns a connection pool.
// It will create a new ephemeral database for the test run, and clean up after itself afterwards.
func CreateTestDB(t testingT, opts ...testDatabaseOption) (*pgxpool.Pool, error) {
	t.Helper()
	ctx := t.Context()

	options := &testDatabaseOptions{
		additionalMigrations: make(map[string]string),
	}
	for _, opt := range opts {
		opt(options)
	}

	// First we connect to the test database that already exists. In it, we will create a new database in it.
	pool, err := Connect(ctx, getTestDatabaseDSN(env.Get("TEST_DB_NAME", "cov")))
	if err != nil {
		return nil, err
	}
	defer pool.Close()

	newDBName := generateDBName()
	_, err = pool.Exec(ctx, fmt.Sprintf("CREATE DATABASE %s", newDBName))
	if err != nil {
		return nil, fmt.Errorf("failed to create test database: %w", err)
	}
	pool.Close()

	// Now we can connect to the new database.
	connString := getTestDatabaseDSN(newDBName)
	pool, err = Connect(ctx, connString)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to test database: %w", err)
	}
	t.Cleanup(pool.Close)

	// We need to clean up the database after the test is done.
	t.Cleanup(func() {
		// If/when this fails, we don't want to fail the test. We just log it.

		pool, err := Connect(ctx, getTestDatabaseDSN(env.Get("TEST_DB_NAME", "cov")))
		if err != nil {
			t.Logf("failed to connect to test database to clean up: %v", err)
			return
		}

		_, err = pool.Exec(ctx, fmt.Sprintf("DROP DATABASE %s", newDBName))
		if err != nil {
			t.Logf("failed to drop test database: %v", err)
		}
	})

	if !options.disableMigrations {
		// Now, let's run the migrations.
		var migrations source.Driver
		if len(options.additionalMigrations) > 0 {
			migrations, err = createMigrationSource(t.TempDir(), options.additionalMigrations)
			if err != nil {
				return nil, fmt.Errorf("failed to create migration source with additional migrations: %w", err)
			}
		} else {
			migrations, err = EmbeddedMigrationsSource()
			if err != nil {
				return nil, fmt.Errorf("failed to get embedded migrations: %w", err)
			}
		}
		defer migrations.Close()

		if err := ExecuteMigrations(ctx, migrations, pool); err != nil {
			return nil, fmt.Errorf("failed to run migrations: %w", err)
		}
	}

	// Now we're ready to run tests!
	return pool, nil
}

func getTestDatabaseDSN(db string) string {
	return fmt.Sprintf(env.Get("DATABASE", "host=localhost port=5432 user=cov password=cov dbname=%s sslmode=disable pool_max_conns=10"), db)
}

func generateDBName() string {
	const alphabet = "abcdefghijklmnopqrstuvwxyz"
	const length = 8
	b := &strings.Builder{}
	b.WriteString("cov_testdb_")
	b.Grow(length)
	for range length {
		b.WriteRune(rune(alphabet[rand.IntN(len(alphabet))]))
	}
	return b.String()
}

func createMigrationSource(dir string, additionalMigrations map[string]string) (source.Driver, error) {
	// Copy the embedded migrations to the directory.
	if err := copyEmbeddedMigrations(dir); err != nil {
		return nil, fmt.Errorf("failed to copy embedded migrations: %w", err)
	}

	// Write the additional migrations to the directory.
	for name, content := range additionalMigrations {
		fileName := filepath.Join(dir, name)
		if !strings.HasSuffix(fileName, ".sql") {
			fileName += ".sql"
		}

		if err := os.WriteFile(fileName, []byte(content), 0o644); err != nil {
			return nil, fmt.Errorf("failed to write additional migration file %s: %w", fileName, err)
		}
	}

	return source.Open("file://" + dir)
}

// copyEmbeddedMigrations copies the embedded migrations to the specified directory.
func copyEmbeddedMigrations(dir string) error {
	dirEntry, err := migrationsFS.ReadDir("migrations")
	if err != nil {
		return fmt.Errorf("failed to read migrations directory: %w", err)
	}

	for _, entry := range dirEntry {
		fileName := entry.Name()
		if entry.IsDir() {
			if err := os.MkdirAll(filepath.Join(dir, fileName), 0o644); err != nil {
				return fmt.Errorf("failed to create directory %s: %w", fileName, err)
			}
			continue
		}

		file, err := migrationsFS.Open(filepath.Join("migrations", fileName))
		if err != nil {
			return fmt.Errorf("failed to open migration file %s: %w", fileName, err)
		}
		// We might not want to defer here... but we don't have many migrations (yet) so it's fine for now.
		defer file.Close()

		destFile, err := os.Create(filepath.Join(dir, fileName))
		if err != nil {
			return fmt.Errorf("failed to create destination file %s: %w", fileName, err)
		}
		// Same here.
		defer destFile.Close()

		if _, err := io.Copy(destFile, file); err != nil {
			return fmt.Errorf("failed to copy migration file %s: %w", fileName, err)
		}
	}

	return nil
}
