CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON users (email);

CREATE UNIQUE INDEX ON users (username);

-- A login token, so as to not require the user to store their password in the client/cookies/etc.
CREATE TABLE user_tokens (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- access_token is a hashed random string. This is a PHC string.
    access_token TEXT NOT NULL,
    expiry TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON user_tokens (user_id);

CREATE TABLE user_oauth2 (
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- service is the service that the user is authenticated with.
    service TEXT NOT NULL,
    -- service_id is the ID of the user on the service.
    service_id TEXT NOT NULL,
    -- access_token is the token that the service uses to identify the user.
    -- It is encrypted in the database with a key provided to the application; without the key, this is gibberish.
    access_token TEXT NOT NULL,
    -- access_token_nonce is the nonce for the access_token.
    access_token_nonce TEXT NOT NULL,
    -- refresh_token is the token that the service uses to refresh the access_token.
    -- It is encrypted in the database with a key provided to the application; without the key, this is gibberish.
    refresh_token TEXT NOT NULL,
    -- refresh_token_nonce is the nonce for the refresh_token.
    refresh_token_nonce TEXT NOT NULL,
    -- expiry is when the token expires. This is set by the service and may be cut short by either party at any time.
    expiry TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON user_oauth2 (user_id, service);

CREATE UNIQUE INDEX ON user_oauth2 (service, service_id);

CREATE INDEX ON user_oauth2 (expiry);
