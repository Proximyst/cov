CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON users (username);

CREATE TABLE user_roles (
    id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- role is the name of the role, matching the RBAC policies.
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, role)
);

CREATE TABLE user_passwords (
    id UUID NOT NULL PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    -- password is a hashed random string. This is a PHC string.
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_emails (
    id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- email is the email address of the user.
    email TEXT NOT NULL,
    -- verified is whether the email address has been verified.
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    -- is_primary is whether this is the primary email address of the user.
    -- There can only be one primary email address per user.
    -- The primary email address is used for password resets and other account-related actions.
    -- All emails may receive any other notifications.
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, email)
);
