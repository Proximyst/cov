# The Justfile is formatted into sections:
#   1. Active dev tooling: stuff you'll use when working.
#   2. Passive dev tooling: stuff you'll use indirectly, e.g. via precommit hooks.
#   3. CI/CD tooling
#   4. Samples

# SECTION: Active dev tooling

# Format all files in the repository.
fmt:
    #!/bin/bash
    set -euo pipefail
    (stdbuf -oL taplo fmt 2>&1                             | sed "s/^/$(printf '\033[33mtaplo    :\033[0m') /") &
    (stdbuf -oL cargo +nightly fmt 2>&1                    | sed "s/^/$(printf '\033[34mcargo fmt:\033[0m') /") &
    (stdbuf -oL sh -c 'cd web && just fmt lint-write' 2>&1 | sed "s/^/$(printf '\033[35mweb fmt  :\033[0m') /") &
    wait

# Prepare SQLx query metadata.
prepare:
    cargo sqlx prepare --workspace

# Run linters on the project.
lint:
    taplo check
    cargo fmt --check
    cargo clippy
    cargo sqlx prepare --workspace --check
    cd web && just lint

# Build the entire project for a production environment.
build:
    cd web && just build
    cargo build --features include-frontend

# Run all tests.
test:
    if cargo nextest --version &>/dev/null; then just _fast_test; else just _legacy_test; fi
    cd web && just test
_fast_test:
    cargo nextest run
    cargo test --doc
_legacy_test:
    cargo test

# Run all tests with code coverage tracking. Assumes cargo, cargo-llvm-cov, and yarn are installed.
test-cov:
    if test -d target/llvm-cov/; then rm -r target/llvm-cov/; fi
    mkdir -p target/llvm-cov/
    cargo llvm-cov
    cargo llvm-cov report --lcov --doctests --output-path target/llvm-cov/lcov.info
    cargo llvm-cov report --html --doctests
    if test -d web/coverage/; then rm -r web/coverage/; fi
    cd web && just test-cov

# Create and ready a development database. Assumes user-level access to Docker (or an alias to podman) exists.
dev-db:
    docker compose down --volumes || true
    docker compose up -d --wait
    cd migrations && just run

# Run cov-server with hot reloading, with proxying to `yarn dev`.
serve *ARGS='--logger cov_server=trace,info':
    cargo watch -w Cargo.toml -w Cargo.lock -w server -w proto -- cargo run --package cov-server --features proxy-dev -- {{ARGS}}

# Run cov-server and frontend server. Assume stdbuf (GNU coreutils), cargo, cargo-watch, and yarn are installed.
dev *ARGS='--logger cov_server=trace,info':
    #!/bin/bash
    set -euo pipefail
    if ! stdbuf --version &>/dev/null; then echo 'stdbuf is missing.'; exit 1; fi
    (stdbuf -oL just serve {{ARGS}} 2>&1 | sed "s/^/$(printf '\033[33mbackend :\033[0m') /") &
    (cd web && stdbuf -oL just dev 2>&1  | sed "s/^/$(printf '\033[34mfrontend:\033[0m') /") &
    trap 'kill $(jobs -pr)' SIGINT
    wait

# Update all dependencies in the repository, except samples.
update:
    cargo update
    cd web && just update

# SECTION: Passive dev tooling

# Set up a precommit hook to ensure all code is formatted and tests passing before committing.
setup-precommit:
    cp assets/pre-commit.sh .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
_precommit:
    just lint test

# SECTION: CI/CD tooling

# SECTION: Samples

# Run and collect samples. Read individual justfiles for assumptions.
samples:
    if stdbuf --version &>/dev/null; then just _parallel_samples; else just _sequential_samples; fi

_samples-c:
    cd samples/c && just
_samples-cpp:
    cd samples/cpp && just
_samples-golang:
    cd samples/golang && just
_samples-java:
    cd samples/java && just
_samples-jest:
    cd samples/jest && just
_samples-kotlin:
    cd samples/kotlin && just
_samples-rust:
    cd samples/rust && just

_parallel_samples:
    #!/bin/bash
    set -eu
    (stdbuf -oL just _samples-c      | sed "s/^/$(printf '\033[33mc     :\033[0m') /") &
    (stdbuf -oL just _samples-cpp    | sed "s/^/$(printf '\033[34mcpp   :\033[0m') /") &
    (stdbuf -oL just _samples-golang | sed "s/^/$(printf '\033[31mgolang:\033[0m') /") &
    (stdbuf -oL just _samples-java   | sed "s/^/$(printf '\033[32mjava  :\033[0m') /") &
    (stdbuf -oL just _samples-jest   | sed "s/^/$(printf '\033[35mjest  :\033[0m') /") &
    (stdbuf -oL just _samples-kotlin | sed "s/^/$(printf '\033[35mkotlin:\033[0m') /") &
    (stdbuf -oL just _samples-rust   | sed "s/^/$(printf '\033[35mrust  :\033[0m') /") &
    trap 'kill $(jobs -pr)' SIGINT
    wait

# Runs samples sequentially. Useful for when you need to find out which sample failed, or want it to go slow.
samples-sequential: && _samples-golang _samples-java _samples-c _samples-cpp _samples-jest _samples-kotlin _samples-rust
