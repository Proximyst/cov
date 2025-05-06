package userdel

import (
	"context"
	"errors"
	"fmt"
	"log/slog"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/db/pgutil"
)

func run(ctx context.Context, username, id string, dbConnString string) error {
	if username == "" && id == "" {
		return errors.New("either username or id must be provided")
	} else if username != "" && id != "" {
		return errors.New("only one of username or id must be provided")
	}

	pool, err := db.Connect(ctx, dbConnString)
	if err != nil {
		return fmt.Errorf("failed to connect to database: %w", err)
	}
	defer pool.Close()

	tx, err := pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	queries := db.New(tx)
	var userID pgtype.UUID
	if id != "" {
		id, err := uuid.Parse(id)
		if err != nil {
			return fmt.Errorf("failed to parse user ID: %w", err)
		}

		userID = pgutil.FromGoogleUUID(id)
	} else {
		usr, err := queries.GetUserByUsername(ctx, username)
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
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	slog.Info("deleted user", "username", username, "id", userID.String())

	return nil
}
