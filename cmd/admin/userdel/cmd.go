package userdel

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/db/pgutil"
)

type Command struct {
	Database db.Flags `embed:""`

	Username string `help:"The username of the user to delete." xor:"username,id" required:""`
	ID       string `help:"The ID of the user to delete." xor:"username,id" required:""`
}

func (c *Command) Run(ctx context.Context) error {
	pool, err := c.Database.Connect(ctx)
	if err != nil {
		return fmt.Errorf("failed to connect to database: %w", err)
	}
	defer pool.Close()

	tx, err := pool.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	queries := db.New(tx)
	var userID pgtype.UUID
	if c.ID != "" {
		id, err := uuid.Parse(c.ID)
		if err != nil {
			return fmt.Errorf("failed to parse user ID: %w", err)
		}

		userID = pgutil.FromGoogleUUID(id)
	} else {
		usr, err := queries.GetUserByUsername(ctx, c.Username)
		if err != nil {
			return fmt.Errorf("failed to get user by username: %w", err)
		}

		userID = usr.ID
	}

	if err := queries.DeleteUser(ctx, userID); err != nil {
		return fmt.Errorf("failed to delete user: %w", err)
	}
	if _, err := queries.CreateAuditLogEvent(ctx, db.CreateAuditLogEventParams{
		EventType: db.AuditLogEventTypeUserDeleted,
		EventData: db.AuditLogEventData{
			UserDeleted: &db.AuditLogUserDeleted{UserID: userID.String()},
		},
	}); err != nil {
		return fmt.Errorf("failed to create audit log user deletion event: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	slog.Info("deleted user", "username", c.Username, "id", userID.String())
	return nil
}
