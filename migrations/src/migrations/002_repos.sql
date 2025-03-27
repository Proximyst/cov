CREATE TABLE organisations (
    id UUID NOT NULL PRIMARY KEY,
    service TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE UNIQUE INDEX ON organisations (service, name);

CREATE TABLE repositories (
    id UUID NOT NULL PRIMARY KEY,
    organisation_id UUID NOT NULL REFERENCES organisations (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE INDEX ON repositories (organisation_id, name);
