package migrate_test

import (
	"testing"

	"github.com/jackc/pgx/v5"
	"github.com/proximyst/cov/cmd/migrate"
	"github.com/proximyst/cov/pkg/db"
	"github.com/stretchr/testify/require"
)

func TestMigrate(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping test in short mode")
	}
	t.Parallel()

	pool, err := db.CreateTestDB(t, db.WithoutMigrations())
	require.NoError(t, err, "failed to create test database")

	cmd := migrate.New()
	err = cmd.Run(t.Context(), []string{"migrate", "--database", pool.Config().ConnString()})
	require.NoError(t, err, "failed to run migrate command")

	// Check if the migrations were applied
	_, err = db.New().GetUserByUsername(t.Context(), pool, "this username does not exist")
	// We expect the error to be ErrNoRows, rather than some other DB error (such as "role not found").
	// When this is the case, the query is correct and table exists but it's just empty, which is totally fine.
	require.ErrorIs(t, err, pgx.ErrNoRows, "expected no user found")
}
