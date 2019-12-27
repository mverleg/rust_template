#!/usr/bin/env bash

(
    # Note that these sets are inside a subshell, so only for this function.
    set -e  # fail if a command fails
    set -E  # technical change so traps work with -E
    set -o pipefail  # also include intermediate commands in -e
    set -u  # undefined variables are errors
    set -v  # show all the commands, without expanding variables

    # Add components.
    if ! rustup component list | grep -q rustfmt
    then
        rustup component add rustfmt
    fi
    if ! rustup component list | grep -q clippy
    then
        rustup component add clippy
    fi
    if ! hash cargo-audit 2>/dev/null
    then
        cargo install cargo-audit
    fi
    if ! hash cargo-outdated 2>/dev/null
    then
        cargo install --force --git https://github.com/kbknapp/cargo-outdated
    fi

    # Check formatting.
    cargo fmt --all -- --check

    # Check suspicious patterns.
    cargo clippy --all-targets --all-features -- -D warnings

    # Build (to test, and prepare for tests).
    cargo build --all --release -D warnings

    # Run all the tests.
    cargo test --all --release

    # Do not run benchmarks as it is too slow to do every time.

    # Try to build the documentation.
    cargo doc

    #TODO @mark: code coverage?
    #TODO @mark: PGO?
)
