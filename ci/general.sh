#!/usr/bin/env bash

##
## General setup that should be imported by each step.
##

# Note that these sets are inside the bash invoked above, so only for this script.
set -e  # fail if a command fails
set -E  # technical change so traps work with -E
set -o pipefail  # also include intermediate commands in -e
set -u  # undefined variables are errors


# Start C-style header guard (because who doesn't miss those?!).
if [[ -z "${IS_GENERAL_HEADER_INCLUDED:-}" ]]
then

export CARGO_FLAGS="-Z unstable-options -Z config-profile"

if [[ ! -f "Cargo.toml" ]]
then
    printf "'Cargo.toml' not found, this is not the project directory?\n" 1>&2
    exit 1
fi

function showrun() {
    echo ">> $@"
    set +e
    "$@"
    set -e
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

function get_profile_executable() {
    # Note: assumes the most recent executable is correct (so no concurrent builds)
    if [[ "$#" -ne 1 ]]
    then
        printf "Provide profile name to get executable path\n" 1>&2
        exit 1
    fi
    profile_dir="$CARGO_TARGET_DIR/$1"
    if [[ ! -d "$profile_dir" ]]
    then
        printf "Profile $1 has no output directory\n" 1>&2
        exit 1
    fi
    printf "%s" "$(find "$profile_dir" -maxdepth 1 -type f -executable -print0 | (xargs -r -0 ls -1 -t || test $? -eq 141) | head -1)"
}

function determine_clippy_fmt_nightly() {
    # Find a nightly version that has clippy and rustfmt, if there is one in the last week.
    #TODO @mark: needs more testing - right now there's no clippy-compatible version
    python << EOF
from requests import get
from sys import stdout
libs = tuple([
    get(url="https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/clippy.json").json(),
    get(url="https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/rustfmt.json").json() ])
for date in sorted(libs[0].keys(), reverse=True):
    for lib in libs:
        if lib[date] is not True:
            break
    else:
        # date for all libs valid
        stdout.write(date)
        break
# no supported version; choose general nightly, which'll probably fail...
stdout.write("nightly")
EOF
}

# If your version of Rust does not support clippy or another component, check which version does at
# https://rust-lang.github.io/rustup-components-history/index.html
# then switch to it using `rustup default nightly-2019-12-20` (using the correct date).

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

# Set up the correct git version
if [[ -z "${RUST_VERSION:-}" ]]
then
    export RUST_VERSION="$(determine_clippy_fmt_nightly)"
    printf "No specific Rust version requested, falling back to %s\n" "$RUST_VERSION"
elif [[ "$RUST_VERSION" == "nightly" ]]
then
    export RUST_VERSION="$(determine_clippy_fmt_nightly)"
    printf "Requested nightly, chose %s in an attempt to support rustfmt and clippy\n" "$RUST_VERSION"
else
    printf "Specific Rust version requested: %s\n" "$RUST_VERSION"
fi
showrun rustup default "$RUST_VERSION"

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

# Check if platform binaries should be made.
DO_PLATFORM_BINARIES=false
if [[ $* == *--platform-binaries* ]]
then
    DO_PLATFORM_BINARIES=true
else
    printf "Use --platform-binaries to produce platform-specific binaries\n"
fi

# Create directory to store reports.
REPORT_DIR="$CARGO_TARGET_DIR/report"
mkdir -p -m 700 "$REPORT_DIR"

# hash sccache 2>/dev/null
if [[ "$RUSTC_WRAPPER" != "sccache" ]]
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
