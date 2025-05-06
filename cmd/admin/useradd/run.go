package useradd

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/proximyst/cov/pkg/auth"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/db/pgutil"
)

func run(ctx context.Context,
	username, password string,
	email string, verifiedEmail, primaryEmail bool,
	roles []string,
	dbConnString string) error {
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
	userID, err := insertUser(ctx, queries, username)
	if err != nil {
		return fmt.Errorf("failed to insert user: %w", err)
	}

	if password != "" {
		if err := insertPassword(ctx, queries, userID, password); err != nil {
			return fmt.Errorf("failed to insert password: %w", err)
		}
	}
	if email != "" {
		if err := insertEmail(ctx, queries, userID, email, verifiedEmail, primaryEmail); err != nil {
			return fmt.Errorf("failed to insert email: %w", err)
		}
	}
	if err := insertRoles(ctx, queries, userID, roles); err != nil {
		return fmt.Errorf("failed to insert roles: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	slog.Info("added user", "username", username, "id", userID.String())

	return nil
}

func insertUser(ctx context.Context, q db.Querier, username string) (pgtype.UUID, error) {
	userID, err := uuid.NewV7()
	if err != nil {
		return pgtype.UUID{}, fmt.Errorf("failed to generate user ID: %w", err)
	}

	usr, err := q.CreateUser(ctx, db.CreateUserParams{
		ID:       pgutil.FromGoogleUUID(userID),
		Username: username,
	})
	if err != nil {
		return pgtype.UUID{}, fmt.Errorf("failed to create user: %w", err)
	}
	if _, err := q.CreateAuditLogEvent(ctx, db.CreateAuditLogEventParams{
		EventType: db.AuditLogEventTypeUserInserted,
		EventData: db.AuditLogEventData{
			UserInserted: &db.AuditLogUserInserted{
				UserID:   userID.String(),
				Username: username,
			},
		},
	}); err != nil {
		return pgtype.UUID{}, fmt.Errorf("failed to create audit log event: %w", err)
	}

	return usr.ID, nil
}

func insertPassword(ctx context.Context, q db.Querier, userID pgtype.UUID, password string) error {
	hashed, err := auth.HashEncoded([]byte(password))
	if err != nil {
		return fmt.Errorf("failed to hash password: %w", err)
	}

	if err := q.CreateUserPassword(ctx, db.CreateUserPasswordParams{
		ID:       userID,
		Password: hashed,
	}); err != nil {
		return fmt.Errorf("failed to create user password: %w", err)
	}
	if _, err := q.CreateAuditLogEvent(ctx, db.CreateAuditLogEventParams{
		EventType: db.AuditLogEventTypeUserPasswordInserted,
		EventData: db.AuditLogEventData{
			UserPasswordInserted: &db.AuditLogUserPasswordInserted{UserID: userID.String()},
		},
	}); err != nil {
		return fmt.Errorf("failed to create audit log event for user password insertion: %w", err)
	}

	return nil
}

func insertEmail(ctx context.Context, q db.Querier, userID pgtype.UUID, email string, verifiedEmail, primaryEmail bool) error {
	if err := q.CreateUserEmail(ctx, db.CreateUserEmailParams{
		ID:        userID,
		Email:     email,
		Verified:  verifiedEmail,
		IsPrimary: primaryEmail,
	}); err != nil {
		return fmt.Errorf("failed to create user email: %w", err)
	}
	if _, err := q.CreateAuditLogEvent(ctx, db.CreateAuditLogEventParams{
		EventType: db.AuditLogEventTypeUserEmailInserted,
		EventData: db.AuditLogEventData{
			UserEmailInserted: &db.AuditLogUserEmailInserted{
				UserID:    userID.String(),
				Email:     email,
				Verified:  verifiedEmail,
				IsPrimary: primaryEmail,
			},
		},
	}); err != nil {
		return fmt.Errorf("failed to create audit log event for user email insertion: %w", err)
	}

	return nil
}

func insertRoles(ctx context.Context, q db.Querier, userID pgtype.UUID, roles []string) error {
	for _, role := range roles {
		if err := insertRole(ctx, q, userID, role); err != nil {
			return fmt.Errorf("failed to insert role %s: %w", role, err)
		}
	}

	return nil
}

func insertRole(ctx context.Context, q db.Querier, userID pgtype.UUID, role string) error {
	if err := q.CreateUserRole(ctx, db.CreateUserRoleParams{
		ID:   userID,
		Role: role,
	}); err != nil {
		return fmt.Errorf("failed to create user role: %w", err)
	}
	if _, err := q.CreateAuditLogEvent(ctx, db.CreateAuditLogEventParams{
		EventType: db.AuditLogEventTypeUserRoleInserted,
		EventData: db.AuditLogEventData{
			UserRoleInserted: &db.AuditLogUserRoleInserted{
				UserID: userID.String(),
				Role:   role,
			},
		},
	}); err != nil {
		return fmt.Errorf("failed to create audit log event for user role insertion: %w", err)
	}

	return nil
}
