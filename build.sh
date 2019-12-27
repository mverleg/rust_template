#!/usr/bin/env bash

function showrun() {
    echo ">> $@"
    if ! "$@"
    then
        printf "*** A PROBLEM OCCURRED WHEN RUNNING: %s ***" "'$*'\n" 1>&2
        exit $?
    fi
}

# Note that these sets are inside the bash invoked above, so only for this script.
set -e  # fail if a command fails
set -E  # technical change so traps work with -E
set -o pipefail  # also include intermediate commands in -e
set -u  # undefined variables are errors
c
# If your version of Rust does not support clippy or another component, check which version does at
# https://rust-lang.github.io/rustup-components-history/index.html
# then switch to it using `rustup default nightly-2019-12-20` (using the correct date).

if ! hash sccache 2>/dev/null
then
    printf "Not using sccache (using sccache is recommended: https://github.com/mozilla/sccache)\n" 1>&2
fi

# Add components.
if ! rustup component list | grep -q rustfmt
then
    showrun rustup component add rustfmt
fi
if ! rustup component list | grep -q clippy
then
    showrun rustup component add clippy
fi
if ! hash cargo-build-deps 2>/dev/null
then
    showrun cargo install cargo-build-deps
fi
if ! hash cargo-audit 2>/dev/null
then
    showrun cargo install cargo-audit
fi
if ! hash cargo-outdated 2>/dev/null
then
    showrun cargo install --force --git https://github.com/kbknapp/cargo-outdated
fi

# Check the dependency versions.
# Note that things can still get outdated *after* release.
showrun cargo audit --deny-warnings
showrun cargo outdated --exit-code 1

# Build dependencies, as they shouldn't affect clippy etc.
showrun cargo build-deps --release 1>/dev/null

# Fix formatting and compiler warnings, if --fix is given
if [[ $* == *--fix* ]]
then
    if [[ -n "$(git status --porcelain)" ]]
    then
        printf "Refusing to apply compiler suggestions and rustfmt fixes as git reports that there are pending changes\n" 1>&2
        exit 1
    fi
    printf "Applying compiler suggestions and rustfmt fixes\n"
    if ! hash cargo-fix 2>/dev/null
    then
        showrun cargo install cargo-fix
    fi
    showrun cargo fmt
    showrun cargo fix --workspace --all-targets --all-features
fi

# Clean already-compiled code for current project(s)
find . -maxdepth 3 -type f -name Cargo.toml | xargs grep '^\s*name\s*=\s*"' | sed -E 's/\s*name\s*=\s*"([^"]*)"\s*/\1/' | sort | uniq | xargs -I'{}' -- bash -c 'echo cargo clean -p {}; cargo clean -p {}'

# Check formatting.
showrun cargo fmt -- --check

# Check suspicious patterns.
showrun cargo clippy --workspace --all-targets --all-features -- -D warnings

# Build (to test, and prepare for tests).
showrun cargo build --workspace --release -- -D warnings

# Run all the tests.
showrun cargo test --workspace --release -- -D warnings

# Do not run benchmarks as it is too slow to do every time.

# Try to build the documentation.
showrun cargo doc --all-features -- -D warnings

if [[ $* == *--fix* ]] && [[ -n "$(git status --porcelain)" ]]
then
    printf "Automatic changes were made, do not forget to commit them!\n"
fi

#TODO @mark: code coverage?
#TODO @mark: PGO?
