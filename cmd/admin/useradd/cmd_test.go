package useradd_test

import (
	"testing"

	"github.com/proximyst/cov/cmd/admin/useradd"
	"github.com/proximyst/cov/pkg/auth"
	"github.com/proximyst/cov/pkg/covtest"
	"github.com/proximyst/cov/pkg/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestUserAdd(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping test in short mode")
	}
	t.Parallel()

	t.Run("inserted user and role", func(t *testing.T) {
		t.Parallel()

		pool, err := db.CreateTestDB(t)
		require.NoError(t, err, "failed to create test database")

		err = covtest.Run(t, &useradd.Command{}, []string{
			"--database", pool.Config().ConnString(),
			"--username", "testuser"})
		require.NoError(t, err, "failed to run the useradd command")

		usr, err := db.New(pool).GetUserByUsername(t.Context(), "testuser")
		require.NoError(t, err, "failed to find user in database")
		assert.Equal(t, "testuser", usr.Username, "username should match")

		events, err := db.New(pool).GetAuditLogEvents(t.Context(), db.GetAuditLogEventsParams{
			Limit: 100,
		})
		require.NoError(t, err, "failed to get audit log events")

		userInserted := eventByType(events, db.AuditLogEventTypeUserInserted)
		if assert.NotNil(t, userInserted, "user inserted event should not be nil") &&
			assert.NotNil(t, userInserted.EventData.UserInserted, "user inserted event data should not be nil") {
			assert.Equal(t, usr.ID.String(), userInserted.EventData.UserInserted.UserID, "user ID should match")
			assert.Equal(t, usr.Username, userInserted.EventData.UserInserted.Username, "username should match")
		}

		userRoleInserted := eventByType(events, db.AuditLogEventTypeUserRoleInserted)
		if assert.NotNil(t, userRoleInserted, "user role inserted event should not be nil") &&
			assert.NotNil(t, userRoleInserted.EventData.UserRoleInserted, "user role inserted event data should not be nil") {
			assert.Equal(t, usr.ID.String(), userRoleInserted.EventData.UserRoleInserted.UserID, "user ID should match")
			assert.Equal(t, "user", userRoleInserted.EventData.UserRoleInserted.Role, "role name should match")
		}
	})

	t.Run("added email", func(t *testing.T) {
		t.Parallel()

		t.Run("verified and primary", func(t *testing.T) {
			t.Parallel()

			pool, err := db.CreateTestDB(t)
			require.NoError(t, err, "failed to create test database")

			err = covtest.Run(t, &useradd.Command{}, []string{
				"--database", pool.Config().ConnString(),
				"--username", "testuser",
				"--email", "testuser@example.localhost"})
			require.NoError(t, err, "failed to run the useradd command")

			usr, err := db.New(pool).GetUserByUsername(t.Context(), "testuser")
			require.NoError(t, err, "failed to find user in database")
			assert.Equal(t, "testuser", usr.Username, "username should match")

			emails, err := db.New(pool).GetUserEmails(t.Context(), db.GetUserEmailsParams{
				ID:    usr.ID,
				Limit: 100,
			})
			require.NoError(t, err, "failed to get user emails")
			require.Len(t, emails, 1, "expected one email")
			assert.Equal(t, usr.ID, emails[0].ID, "user ID should match")
			assert.Equal(t, "testuser@example.localhost", emails[0].Email, "email should match")
			assert.True(t, emails[0].Verified, "email should be verified")
			assert.True(t, emails[0].IsPrimary, "email should be primary")

			events, err := db.New(pool).GetAuditLogEvents(t.Context(), db.GetAuditLogEventsParams{
				Limit: 100,
			})
			require.NoError(t, err, "failed to get audit log events")

			userEmailInserted := eventByType(events, db.AuditLogEventTypeUserEmailInserted)
			if assert.NotNil(t, userEmailInserted, "user email inserted event should not be nil") &&
				assert.NotNil(t, userEmailInserted.EventData.UserEmailInserted, "user email inserted event data should not be nil") {
				assert.Equal(t, usr.ID.String(), userEmailInserted.EventData.UserEmailInserted.UserID, "user ID should match")
				assert.Equal(t, "testuser@example.localhost", userEmailInserted.EventData.UserEmailInserted.Email, "email should match")
				assert.True(t, userEmailInserted.EventData.UserEmailInserted.Verified, "email should be verified")
				assert.True(t, userEmailInserted.EventData.UserEmailInserted.IsPrimary, "email should be primary")
			}
		})

		t.Run("verified and primary", func(t *testing.T) {
			t.Parallel()

			pool, err := db.CreateTestDB(t)
			require.NoError(t, err, "failed to create test database")

			err = covtest.Run(t, &useradd.Command{}, []string{
				"--database", pool.Config().ConnString(),
				"--username", "testuser",
				"--email", "testuser@example.localhost",
				"--verified-email=false",
				"--primary-email=false"})
			require.NoError(t, err, "failed to run the useradd command")

			usr, err := db.New(pool).GetUserByUsername(t.Context(), "testuser")
			require.NoError(t, err, "failed to find user in database")
			assert.Equal(t, "testuser", usr.Username, "username should match")

			emails, err := db.New(pool).GetUserEmails(t.Context(), db.GetUserEmailsParams{
				ID:    usr.ID,
				Limit: 100,
			})
			require.NoError(t, err, "failed to get user emails")
			require.Len(t, emails, 1, "expected one email")
			assert.Equal(t, usr.ID, emails[0].ID, "user ID should match")
			assert.Equal(t, "testuser@example.localhost", emails[0].Email, "email should match")
			assert.False(t, emails[0].Verified, "email should not be verified")
			assert.False(t, emails[0].IsPrimary, "email should not be primary")

			events, err := db.New(pool).GetAuditLogEvents(t.Context(), db.GetAuditLogEventsParams{
				Limit: 100,
			})
			require.NoError(t, err, "failed to get audit log events")

			userEmailInserted := eventByType(events, db.AuditLogEventTypeUserEmailInserted)
			if assert.NotNil(t, userEmailInserted, "user email inserted event should not be nil") &&
				assert.NotNil(t, userEmailInserted.EventData.UserEmailInserted, "user email inserted event data should not be nil") {
				assert.Equal(t, usr.ID.String(), userEmailInserted.EventData.UserEmailInserted.UserID, "user ID should match")
				assert.Equal(t, "testuser@example.localhost", userEmailInserted.EventData.UserEmailInserted.Email, "email should match")
				assert.False(t, userEmailInserted.EventData.UserEmailInserted.Verified, "email should not be verified")
				assert.False(t, userEmailInserted.EventData.UserEmailInserted.IsPrimary, "email should not be primary")
			}
		})
	})

	t.Run("added password", func(t *testing.T) {
		t.Parallel()

		pool, err := db.CreateTestDB(t)
		require.NoError(t, err, "failed to create test database")

		err = covtest.Run(t, &useradd.Command{}, []string{
			"--database", pool.Config().ConnString(),
			"--username", "testuser",
			"--password", "testpassword"})
		require.NoError(t, err, "failed to run the useradd command")

		usr, err := db.New(pool).GetUserWithOptionalPasswordByUsername(t.Context(), "testuser")
		require.NoError(t, err, "failed to find user in database")

		matches, err := auth.VerifyEncoded([]byte("testpassword"), *usr.Password)
		require.NoError(t, err, "failed to verify password")
		assert.True(t, matches, "password should match")

		events, err := db.New(pool).GetAuditLogEvents(t.Context(), db.GetAuditLogEventsParams{
			Limit: 100,
		})
		require.NoError(t, err, "failed to get audit log events")

		userPasswordInserted := eventByType(events, db.AuditLogEventTypeUserPasswordInserted)
		if assert.NotNil(t, userPasswordInserted, "user password inserted event should not be nil") &&
			assert.NotNil(t, userPasswordInserted.EventData.UserPasswordInserted, "user password inserted event data should not be nil") {
			assert.Equal(t, usr.ID.String(), userPasswordInserted.EventData.UserPasswordInserted.UserID, "user ID should match")
		}
	})
}

func eventByType(events []*db.AuditLogEvent, eventType string) *db.AuditLogEvent {
	for _, event := range events {
		if event.EventType == eventType {
			return event
		}
	}
	return nil
}
