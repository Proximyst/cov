# Format all files in the repository. Assumes cargo is installed. Taplo is optional but recommended.
fmt:
    taplo --version &>/dev/null && taplo fmt || true
    cargo +nightly fmt

# Build the entire project. Assumes cargo is installed.
build:
    cargo build

# Run all tests. Assumes cargo is installed. cargo-nextest is optional but recommended for faster test suites.
test:
    if cargo nextest --version &>/dev/null; then just _fast_test; else just _legacy_test; fi

_fast_test:
    cargo nextest run
    cargo test --doc

_legacy_test:
    cargo test

# Run all tests with code coverage tracking. Assumes cargo and cargo-llvm-cov are installed.
test-cov:
    if test -d target/llvm-cov/; then rm -r target/llvm-cov/; fi
    mkdir -p target/llvm-cov/
    cargo llvm-cov
    cargo llvm-cov report --lcov --doctests --output-path target/llvm-cov/lcov.info
    cargo llvm-cov report --json --doctests --output-path target/llvm-cov/cov.json
    cargo llvm-cov report --html --doctests

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

_parallel_samples:
    #!/bin/bash
    set -eu
    (stdbuf -oL just _samples-c      | sed "s/^/$(printf '\033[33mc     :\033[0m') /") &
    (stdbuf -oL just _samples-cpp    | sed "s/^/$(printf '\033[34mcpp   :\033[0m') /") &
    (stdbuf -oL just _samples-golang | sed "s/^/$(printf '\033[31mgolang:\033[0m') /") &
    (stdbuf -oL just _samples-java   | sed "s/^/$(printf '\033[32mjava  :\033[0m') /") &
    (stdbuf -oL just _samples-jest   | sed "s/^/$(printf '\033[35mjest  :\033[0m') /") &
    (stdbuf -oL just _samples-kotlin | sed "s/^/$(printf '\033[35mkotlin:\033[0m') /") &
    trap 'kill $(jobs -pr)' SIGINT
    wait

# Runs samples sequentially. Useful for when you need to find out which sample failed, or want it to go slow.
samples-sequential: && _samples-golang _samples-java _samples-c _samples-cpp _samples-jest _samples-kotlin
