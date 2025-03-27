CREATE TABLE reports (
    id UUID NOT NULL PRIMARY KEY,
    -- the repository that the report is for.
    repository_id UUID NOT NULL REFERENCES repositories (id) ON DELETE CASCADE,
    -- the commit hash of the report.
    commit TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE INDEX ON reports (repository_id, commit);

CREATE INDEX ON reports (repository_id, created_at);

CREATE TABLE report_flags (
    report_id UUID NOT NULL REFERENCES reports (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    PRIMARY KEY (report_id, name)
);

CREATE TABLE report_files (
    id UUID NOT NULL PRIMARY KEY,
    repository_id UUID NOT NULL REFERENCES repositories (id) ON DELETE CASCADE,
    report_id UUID NOT NULL REFERENCES reports (id) ON DELETE CASCADE,
    -- The path to the file in the repository.
    file_path TEXT NOT NULL,
    -- The file's overall coverage in percentage. This is a float between 0 and 100.
    coverage NUMERIC NOT NULL CHECK (
        coverage >= 0
        AND coverage <= 100
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE UNIQUE INDEX ON report_files (repository_id, file_path, report_id);

-- Not all report_files have lines. This is because we clean up old data to keep the database size down.
CREATE TABLE report_file_line_regions (
    id UUID NOT NULL PRIMARY KEY,
    repository_id UUID NOT NULL REFERENCES repositories (id) ON DELETE CASCADE,
    report_file_id UUID NOT NULL REFERENCES report_files (id) ON DELETE CASCADE,
    -- lines are 1-indexed, like in an editor.
    start_line INT NOT NULL CHECK (start_line >= 1),
    end_line INT NOT NULL CHECK (end_line >= start_line),
    -- some tools might let columns start at 0 (e.g. on empty lines).
    start_column INT NOT NULL CHECK (start_column >= 0),
    end_column INT NOT NULL CHECK (end_column >= 0),
    -- unique statements in this region.
    statements INT NOT NULL CHECK (statements >= 0),
    -- the count of times a statement was executed.
    -- the same statement might be executed multiple times.
    executed INT NOT NULL CHECK (executed >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE INDEX ON report_file_line_regions (repository_id, report_file_id);
