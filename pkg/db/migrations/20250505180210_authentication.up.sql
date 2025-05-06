CREATE TABLE user_sessions (
    id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    session_token TEXT NOT NULL,
    expiry TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, session_token)
);

CREATE UNIQUE INDEX ON user_sessions (session_token);
