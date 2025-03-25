CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY,
    service TEXT NOT NULL,
    -- the service_id is the ID that the service uses to identify the user.
    -- it is opaque to the application, and not unique across services.
    service_id TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    -- created_at is when the user first logged into cov, not on the service.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE UNIQUE INDEX ON users (service, service_id);

CREATE TABLE user_tokens (
    id UUID NOT NULL PRIMARY KEY,
    -- the user_id is the ID of the user that the token is for.
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- the service_token is the token that the service uses to identify the user.
    -- It is encrypted in the database with a key provided to the application; without the key, this is gibberish.
    service_token bytea NOT NULL,
    -- the application_token is a random token that is used to authenticate the user.
    -- It is encrypted in the database with a key provided to the application; without the key, this is gibberish.
    application_token bytea NOT NULL,
    -- expiry is when the token expires.
    -- This must be less than or equal to the service_token's expiry, though it may be cut short at any given time by logging out or being revoked.
    expiry TIMESTAMPTZ NOT NULL DEFAULT NOW () + INTERVAL '7 days',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE INDEX ON user_tokens (user_id);

CREATE INDEX ON user_tokens (expiry);
