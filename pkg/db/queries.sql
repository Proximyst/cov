-- name: CreateAuditLogEvent :one
-- Creates a new audit log event with the given type and data.
INSERT INTO audit_log_events (event_type, event_data)
VALUES ($1, $2)
RETURNING id;

-- name: GetAuditLogEvents :many
-- Gets all audit log events in a page.
SELECT
    id,
    event_type,
    event_data,
    created_at
FROM audit_log_events
ORDER BY id ASC
LIMIT $1 OFFSET $2;

-- name: CreateUser :one
-- Creates a new user with the given ID and username.
INSERT INTO users (id, username)
VALUES ($1, $2)
RETURNING id, username, created_at, updated_at;

-- name: CreateUserEmail :exec
-- Creates a new user email with the given ID and email address.
INSERT INTO user_emails (id, email, verified, is_primary)
VALUES ($1, $2, $3, $4)
ON CONFLICT (id, email) DO UPDATE
    SET
        verified = excluded.verified,
        is_primary = excluded.is_primary;

-- name: CreateUserPassword :exec
-- Creates a new user password with the given ID and password hash.
INSERT INTO user_passwords (id, password)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE
    SET password = excluded.password;

-- name: CreateUserRole :exec
-- Creates a new user role with the given ID and role name.
INSERT INTO user_roles (id, role)
VALUES ($1, $2)
ON CONFLICT (id, role) DO NOTHING;

-- name: DeleteUser :exec
-- Deletes a user with the given ID.
DELETE FROM users
WHERE id = $1;

-- name: GetUserByID :one
-- Gets a user by their ID.
SELECT
    users.id,
    users.username,
    users.created_at,
    users.updated_at
FROM users
WHERE users.id = $1;

-- name: GetUserByUsername :one
-- Gets a user by their username.
SELECT
    users.id,
    users.username,
    users.created_at,
    users.updated_at
FROM users
WHERE users.username = $1;

-- name: GetUserByToken :one
-- Gets a user by one of their session tokens.
SELECT
    users.id,
    users.username,
    ARRAY_AGG(user_roles.role)::TEXT [] AS roles
FROM user_sessions
INNER JOIN users ON user_sessions.id = users.id
INNER JOIN user_roles ON users.id = user_roles.id
WHERE user_sessions.session_token = $1
GROUP BY users.id, users.username;

-- name: GetUserWithOptionalPasswordByUsername :one
-- Gets a user by their username, optionally including their password hash.
SELECT
    users.id,
    user_passwords.password
FROM users
LEFT JOIN user_passwords ON users.id = user_passwords.id
WHERE users.username = $1;

-- name: CreateUserSession :one
-- Creates a new user session with the given ID and session token.
INSERT INTO user_sessions (id, session_token, expiry)
VALUES ($1, $2, $3)
RETURNING id, session_token, expiry;

-- name: GetUserEmails :many
-- Gets all emails for a user with the given ID in a page.
SELECT
    user_emails.id,
    user_emails.email,
    user_emails.verified,
    user_emails.is_primary
FROM user_emails
WHERE user_emails.id = $1
ORDER BY user_emails.is_primary DESC, user_emails.email ASC
LIMIT $2 OFFSET $3;
