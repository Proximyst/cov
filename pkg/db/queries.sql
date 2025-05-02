-- name: GetUserByID :one
-- Gets a user by their ID.
SELECT
    id,
    email,
    username,
    display_name,
    password,
    created_at,
    updated_at
FROM users
WHERE id = $1;

-- name: GetUserByEmail :one
-- Gets a user by their email.
SELECT
    id,
    email,
    username,
    display_name,
    password,
    created_at,
    updated_at
FROM users
WHERE email = $1;

-- name: GetUserByUsername :one
-- Gets a user by their username.
SELECT
    id,
    email,
    username,
    display_name,
    password,
    created_at,
    updated_at
FROM users
WHERE username = $1;
