#!/usr/bin/env bash

##
## General setup that should be imported by each step.
##

# If your version of Rust does not support clippy or another component, check which version does at
# https://rust-lang.github.io/rustup-components-history/index.html
# then switch to it using `rustup default nightly-2019-12-20` (using the correct date).

# Start C-style header guard (because who doesn't miss those?!).
if [[ -z "${IS_GENERAL_HEADER_INCLUDED:-}" ]]
then

# Note that these sets are inside the bash invoked above, so only for this script.
set -e  # fail if a command fails
set -E  # technical change so traps work with -E
set -o pipefail  # also include intermediate commands in -e
set -u  # undefined variables are errors

function showrun() {
    echo ">> $@"
    "$@"
    exit_status="$?"
    if [[ "$exit_status" -ne "0" ]]
    then
        printf "*** A PROBLEM OCCURRED WHEN RUNNING: %s ***\n" "'$*'" 1>&2
        exit $exit_status
    fi
}

function clean_own_code_targets() {
    find . -maxdepth 5 -type f -name 'Cargo.toml' |
        xargs grep '^\s*name\s*=\s*"' |
        sed -E 's/\s*name\s*=\s*"([^"]*)"\s*/\1/' |
        sort | uniq |
        xargs -I"{}" bash -c 'printf "cargo clean -p {}\n"; cargo clean -p {}'
}

export CARGO_FLAGS="-Z unstable-options -Z config-profile"

if [[ ! -f "Cargo.toml" ]]
then
    printf "'Cargo.toml' not found, this is not the project directory?\n" 1>&2
    exit 1
fi

# Set target dir, if not set, so commands can use it.
if [[ -z "${CARGO_TARGET_DIR:-}" ]]
then
    export CARGO_TARGET_DIR="$(pwd)/target"
    mkdir -p -m 700 "$CARGO_TARGET_DIR"
fi

# Make sure library path exists, so -u doesn't crash it
if [[ -z "${LD_LIBRARY_PATH:-}" ]]
then
    export LD_LIBRARY_PATH=""
fi

# Check if automatic fixes should be applied.
DO_FIX=false
if [[ $* == *--fix* ]]
then
    DO_FIX=true
        if [[ -n "$(git status --porcelain)" ]]
    then
        printf "Refusing to apply automatic fixes, because git reports that there are pending changes\n" 1>&2
        print "(to override, use --force-fix instead)\n" 1>&2
        git status --short
        exit 1
    fi
elif [[ $* == *--force-fix* ]]
then
    DO_FIX=true
fi

# Create directory to store reports.
REPORT_DIR="$CARGO_TARGET_DIR/report"
mkdir -p -m 700 "$REPORT_DIR"

if ! hash sccache 2>/dev/null
then
    printf "Not using sccache (using sccache is recommended: https://github.com/mozilla/sccache)\n" 1>&2
fi

# Add components.
if ! hash grcov 2>/dev/null
then
    showrun cargo $CARGO_FLAGS install grcov
fi

# End of the C-style header guard.
IS_GENERAL_HEADER_INCLUDED=1
fi
