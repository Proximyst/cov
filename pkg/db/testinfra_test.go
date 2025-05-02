package db

import (
	"testing"

	"github.com/stretchr/testify/require"
)

var _ testingT = (testing.TB)(nil)

func TestCreatingDB(t *testing.T) {
	t.Parallel()

	t.Run("with default options", func(t *testing.T) {
		t.Parallel()

		// Ensure that we can actually create a database. It doesn't need to do anything further.
		// This also happens to smoke test that the default migrations actually work.
		pool, err := CreateTestDB(t)
		require.NoError(t, err, "failed to create test database")

		rows, err := pool.Query(t.Context(), "SELECT 1")
		require.NoError(t, err, "failed to query test database")
		rows.Close()
	})

	t.Run("with additional migrations", func(t *testing.T) {
		t.Parallel()

		pool, err := CreateTestDB(t,
			WithAdditionalMigration("1_test.up.sql", "CREATE TABLE test (val INT)"),
			WithAdditionalMigration("2_add_to_test.up.sql", "INSERT INTO test (val) VALUES (1)"))
		require.NoError(t, err, "failed to create test database with additional migrations")

		var i int
		err = pool.QueryRow(t.Context(), "SELECT val FROM test").Scan(&i)
		require.NoError(t, err, "failed to query test database with additional migrations")
		require.Equal(t, 1, i, "unexpected value from test table")
	})
}
