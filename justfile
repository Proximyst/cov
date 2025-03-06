# Format all files in the repository. Assumes cargo, taplo, and buf are installed.
fmt:
    taplo fmt
    cargo fmt
    buf format --write

# Build the entire project. Assumes cargo is installed.
build:
    cargo build

# Run all tests. Assumes cargo is installed. cargo-nextest is optional but recommended for faster test suites.
test:
    if cargo nextest --version >/dev/null 2>/dev/null; then just _fast_test; else just _legacy_test; fi

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
