#!/usr/bin/env bash

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

if [[ -z "$CARGO_TARGET_DIR" ]]
then
    export CARGO_TARGET_DIR="$(pwd)/target"
fi

# Note that these sets are inside the bash invoked above, so only for this script.
set -e  # fail if a command fails
set -E  # technical change so traps work with -E
set -o pipefail  # also include intermediate commands in -e
set -u  # undefined variables are errors

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
    showrun cargo $CARGOFLAGS install cargo-build-deps
fi
if ! hash cargo-tree 2>/dev/null
then
    showrun cargo $CARGOFLAGS install cargo-tree
fi
if ! hash cargo-audit 2>/dev/null
then
    showrun cargo $CARGOFLAGS install cargo-audit
fi
if ! hash cargo-outdated 2>/dev/null
then
    showrun cargo $CARGOFLAGS install --force --git https://github.com/kbknapp/cargo-outdated
fi
if ! hash cargo-deny 2>/dev/null
then
    showrun cargo $CARGOFLAGS install cargo-deny
fi
if ! hash grcov 2>/dev/null
then
    showrun cargo $CARGOFLAGS install grcov
fi

# Check the dependency versions.
# Note that things can still get outdated *after* release.
#TODO @mark: check all Cargo.toml files instead of only the main one
#TODO @mark: $ stat: cannot stat '/home/mark/rust_template/target/dependencies-checked': No such file or directory
#TODO @mark: $ #./build.sh: line 85: 1577531651 - : syntax error: operand expected (error token is "- ")
if [[ ! -d "$CARGO_TARGET_DIR" ]] ||
    [[ ! -f "$CARGO_TARGET_DIR/dependencies-checked" ]] ||
    [[ $(($(date +%s) - $(stat -c '%Y' "$CARGO_TARGET_DIR/dependencies-checked"))) -gt 3600 ]] ||
    [[ $(($(stat -c '%Y' "Cargo.toml") - $(stat -c '%Y' "$CARGO_TARGET_DIR/dependencies-checked"))) -gt 0 ]]
then
    showrun cargo $CARGOFLAGS audit --deny-warnings
    showrun cargo $CARGOFLAGS outdated --exit-code 1
    showrun cargo $CARGOFLAGS deny check licenses
    showrun cargo $CARGOFLAGS deny check advisories
    showrun cargo $CARGOFLAGS deny check bans  # mostly for checking duplicates
    if [[ -d "$CARGO_TARGET_DIR" ]]; then touch "$CARGO_TARGET_DIR/dependencies-checked"; fi
else
    printf "Skipping dependency checks, because they were already done within the last hour\n"
fi

# Build dependencies, as they shouldn't affect clippy etc.
#showrun cargo $CARGOFLAGS build-deps --release 1>/dev/null

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
        showrun cargo $CARGOFLAGS install cargo-fix
    fi
    showrun cargo $CARGOFLAGS fmt
    showrun cargo $CARGOFLAGS fix --clippy --workspace --all-targets --all-features -Z unstable-options
fi

# Build (to test, and prepare for tests).
#TODO @mark: can this fail on warnings?
clean_own_code_targets
showrun cargo $CARGOFLAGS build --workspace

# Check formatting.
clean_own_code_targets
showrun cargo $CARGOFLAGS fmt -- --check

# Check suspicious patterns.
showrun cargo $CARGOFLAGS clippy --workspace --all-targets --all-features --tests -- -D warnings

# Run all the tests.
# The flags are needed to get reliable coverage results
#TODO @mark: I only want these flags for own code, otherwise all dependencies have to be recompiled
#showrun export RUSTFLAGS="'-Zincremental=0 -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads'"  #TODO @mark: error: unknown debugging option: `no-landing-pads'`
#export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
#printf ">> export RUSTFLAGS=%s\n" "$RUSTFLAGS"
#showrun export CARGO_INCREMENTAL=1
#if [[ -n "$RUSTFLAGS" ]]; then cov_flags="$RUSTFLAGS $cov_flags"; fi  #TODO @mark: don't ignore existing RUSTFLAGS
RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" showrun cargo $CARGOFLAGS test --profile test_coverage --workspace
#showrun export RUSTFLAGS=""  #TODO @mark: don't ignore existing RUSTFLAGS
#TODO @mark: extract
#TODO @mark: can this fail on warnings?

# Do not run benchmarks as it is too slow to do every time.

# Try to build the documentation.
showrun cargo $CARGOFLAGS doc --all-features

# Export some information
mkdir -p -m 700 "$CARGO_TARGET_DIR/report"
cargo $CARGOFLAGS tree > "$CARGO_TARGET_DIR/report/dependencies.txt"

if [[ $* == *--fix* ]] && [[ -n "$(git status --porcelain)" ]]
then
    printf "Automatic changes were made, do not forget to commit them!\n"
fi

#TODO @mark: prevent duplicate dependencies
#TODO @mark: create a --release artifact?
#TODO @mark: code coverage?
#TODO @mark: PGO?
#TODO @mark: create debian packages using cargo-deb ?


