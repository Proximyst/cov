package db

import "time"

const (
	AuditLogEventTypeUserInserted         = "user.inserted"
	AuditLogEventTypeUserDeleted          = "user.deleted"
	AuditLogEventTypeUserPasswordInserted = "user.password.inserted"
	AuditLogEventTypeUserEmailInserted    = "user.email.inserted"
	AuditLogEventTypeUserRoleInserted     = "user.role.inserted"
	AuditLogEventTypeUserSessionInserted  = "user.session.inserted"
)

type AuditLogEventData struct {
	// By is the user who performed the action.
	// This is not necessarily a record in the users table, but could be a reference to a CLI or similar.
	By string `json:"by"`

	UserInserted         *AuditLogUserInserted         `json:"user_inserted,omitempty"`
	UserDeleted          *AuditLogUserDeleted          `json:"user_deleted,omitempty"`
	UserPasswordInserted *AuditLogUserPasswordInserted `json:"user_password_inserted,omitempty"`
	UserEmailInserted    *AuditLogUserEmailInserted    `json:"user_email_inserted,omitempty"`
	UserRoleInserted     *AuditLogUserRoleInserted     `json:"user_role_inserted,omitempty"`
	UserSessionInserted  *AuditLogUserSessionInserted  `json:"user_session_inserted,omitempty"`
}

type AuditLogUserInserted struct {
	UserID   string `json:"user_id"`
	Username string `json:"username"`
}

type AuditLogUserDeleted struct {
	UserID string `json:"user_id"`
}

type AuditLogUserPasswordInserted struct {
	UserID string `json:"user_id"`
}

type AuditLogUserEmailInserted struct {
	UserID    string `json:"user_id"`
	Email     string `json:"email"`
	Verified  bool   `json:"verified"`
	IsPrimary bool   `json:"is_primary"`
}

type AuditLogUserRoleInserted struct {
	UserID string `json:"user_id"`
	Role   string `json:"role"`
}

type AuditLogUserSessionInserted struct {
	UserID string    `json:"user_id"`
	Token  string    `json:"token"`
	Expiry time.Time `json:"expiry"`
	Method string    `json:"method"`
}
