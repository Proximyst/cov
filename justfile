# Format all files in the repository. Assumes cargo and yarn are installed. Taplo is optional but recommended.
fmt:
    taplo --version &>/dev/null && taplo fmt || true
    cargo +nightly fmt
    cd web && just fmt

# Run linters on the project. Assumes cargo and yarn are installed. Taplo is optional but recommended.
lint:
    taplo --version &>/dev/null && taplo check || true
    cargo clippy
    cd web && just lint

# Build the entire project. Assumes cargo and yarn are installed.
build:
    cargo build
    cd web && just build

# Run all tests. Assumes cargo and yarn are installed. cargo-nextest is optional but recommended for faster test suites.
test:
    if cargo nextest --version &>/dev/null; then just _fast_test; else just _legacy_test; fi
    cd web && just test

# Run cov-server with hot reloading. Assumes cargo and cargo-watch are installed.
serve *ARGS='--logger cov_server=trace,info':
    cargo watch -w server -w proto -- cargo run --package cov-server -- {{ARGS}}

# Run cov-server and frontend server. Assume stdbuf (GNU coreutils), cargo, cargo-watch, and yarn are installed.
dev *ARGS='--logger cov_server=trace,info':
    #!/bin/bash
    set -eu
    if ! stdbuf --version &>/dev/null; then echo 'stdbuf is missing.'; exit 1; fi
    (stdbuf -oL just serve {{ARGS}} 2>&1 | sed "s/^/$(printf '\033[33mbackend :\033[0m') /") &
    (cd web && stdbuf -oL just dev  2>&1 | sed "s/^/$(printf '\033[34mfrontend:\033[0m') /") &
    trap 'kill $(jobs -pr)' SIGINT
    wait

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
    cd web && just test-cov

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
