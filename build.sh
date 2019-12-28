#!/usr/bin/env bash



# Build dependencies, as they shouldn't affect clippy etc.
#showrun cargo $CARGOFLAGS build-deps --release 1>/dev/null

# Fix formatting and compiler warnings, if --fix is given
if [[ $* == *--fix* ]]
then
    if [[ -n "$(git status --porcelain)" ]]
    then
        printf "Refusing to apply compiler suggestions and rustfmt fixes as git reports that there are pending changes\n" 1>&2
        git status --short
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

if [[ $* == *--fix* ]] && [[ -n "$(git status --porcelain)" ]]
then
    printf "Automatic changes were made, do not forget to commit them!\n"
fi

#TODO @mark: prevent duplicate dependencies
#TODO @mark: create a --release artifact?
#TODO @mark: code coverage?
#TODO @mark: PGO?
#TODO @mark: create debian packages using cargo-deb ?


