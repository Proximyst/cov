# Run all appropriate linters in a fixing mode.
lint:
    zizmor .
    actionlint
    goimports -w .
    go mod tidy

# Run all tests.
test:
    go test ./...

# Run all tests, always. This skips the test cache.
test-all:
    go test -count=1 ./...

# Build the binary. This produces a production-ready result.
build:
    go build -o ./cov

# Run the binary.
run *ARGS: build
    ./cov {{ARGS}}

# Create and ready a development database. Assumes user-level access to Docker (or an alias to podman) exists.
dev-db:
    docker compose down --volumes || true
    docker compose up -d --wait
    just run migrate

# Set up a Git pre-commit hook to run (fast) linters before committing.
# This is a one-time setup step.
# Slow linters (e.g. tests) are not included in the pre-commit hook.
setup-precommit:
    echo "#!/bin/sh" > .git/hooks/pre-commit
    echo "set -e" >> .git/hooks/pre-commit
    echo "just _precommit" >> .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit

_precommit:
    zizmor .
    actionlint
    goimports -l .
    go mod tidy -diff
