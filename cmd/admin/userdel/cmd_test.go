package userdel_test

import (
	"testing"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/proximyst/cov/cmd/admin/userdel"
	"github.com/proximyst/cov/pkg/covtest"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/db/pgutil"
	"github.com/stretchr/testify/require"
)

func TestUserDel(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping test in short mode")
	}
	t.Parallel()

	t.Run("delete existing user by username", func(t *testing.T) {
		t.Parallel()

		pool, err := db.CreateTestDB(t)
		require.NoError(t, err, "failed to create test database")

		userID, err := uuid.NewV7()
		require.NoError(t, err, "failed to create user ID")
		_, err = db.New(pool).CreateUser(t.Context(), db.CreateUserParams{
			ID:       pgutil.FromGoogleUUID(userID),
			Username: "testuser",
		})
		require.NoError(t, err, "failed to create test user")

		err = covtest.Run(t, &userdel.Command{}, []string{
			"--database", pool.Config().ConnString(),
			"--username", "testuser"})
		require.NoError(t, err, "failed to run the userdel command")

		_, err = db.New(pool).GetUserByUsername(t.Context(), "testuser")
		require.ErrorIs(t, err, pgx.ErrNoRows, "failed to find user in database")
	})

	t.Run("delete existing user by ID", func(t *testing.T) {
		t.Parallel()

		pool, err := db.CreateTestDB(t)
		require.NoError(t, err, "failed to create test database")

		userID, err := uuid.NewV7()
		require.NoError(t, err, "failed to create user ID")
		_, err = db.New(pool).CreateUser(t.Context(), db.CreateUserParams{
			ID:       pgutil.FromGoogleUUID(userID),
			Username: "testuser",
		})
		require.NoError(t, err, "failed to create test user")

		err = covtest.Run(t, &userdel.Command{}, []string{
			"--database", pool.Config().ConnString(),
			"--id", userID.String()})
		require.NoError(t, err, "failed to run the userdel command")

		_, err = db.New(pool).GetUserByUsername(t.Context(), "testuser")
		require.ErrorIs(t, err, pgx.ErrNoRows, "failed to find user in database")
	})
}
