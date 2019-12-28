#!/usr/bin/env bash

##
## General setup that should be imported by each step.
## Should be idempotent.
##

# Note that these sets are inside the bash invoked above, so only for this script.
set -e  # fail if a command fails
set -E  # technical change so traps work with -E
set -o pipefail  # also include intermediate commands in -e
set -u  # undefined variables are errors

function showrun() {
    echo ">> $@"
    if ! "$@"
    then
        printf "*** A PROBLEM OCCURRED WHEN RUNNING: %s ***\n" "'$*'" 1>&2
        exit $?
    fi
}

function clean_own_code_targets() {
    find . -maxdepth 3 -type f -name Cargo.toml | xargs grep '^\s*name\s*=\s*"' |
        sed -E 's/\s*name\s*=\s*"([^"]*)"\s*/\1/' |
        sort | uniq |
        xargs -I'{}' -- bash -c 'echo cargo clean -p {}; cargo clean -p {}'
}

export CARGOFLAGS="-Z unstable-options -Z config-profile"

if [[ ! -f "Cargo.toml" ]]
then
    printf "'Cargo.toml' not found, this is not the project directory?\n" 1>&2
    exit 1
fi

if [[ -z "${CARGO_TARGET_DIR:-}" ]]
then
    export CARGO_TARGET_DIR="$(pwd)/target"
fi

# Create report directory
mkdir -p -m 700 "$CARGO_TARGET_DIR/report"

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
if ! hash grcov 2>/dev/null
then
    showrun cargo $CARGOFLAGS install grcov
fi
