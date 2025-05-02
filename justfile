# Run all appropriate linters in a fixing mode.
lint:
    zizmor .
    actionlint
    goimports -w .

# Run all tests.
test:
    go test ./...

# Run all tests, always. This skips the test cache.
test-all:
    go test -count=1 ./...

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