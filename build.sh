#!/usr/bin/env bash


# Build dependencies, as they shouldn't affect clippy etc.
#showrun cargo $CARGO_FLAGS build-deps --release 1>/dev/null

# Run all the tests.
# The flags are needed to get reliable coverage results
#TODO @mark: I only want these flags for own code, otherwise all dependencies have to be recompiled
#showrun export RUSTFLAGS="'-Zincremental=0 -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads'"  #TODO @mark: error: unknown debugging option: `no-landing-pads'`
#export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
#printf ">> export RUSTFLAGS=%s\n" "$RUSTFLAGS"
#showrun export CARGO_INCREMENTAL=1
#if [[ -n "$RUSTFLAGS" ]]; then cov_flags="$RUSTFLAGS $cov_flags"; fi  #TODO @mark: don't ignore existing RUSTFLAGS
RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" showrun cargo $CARGO_FLAGS test --profile test_coverage --workspace
#showrun export RUSTFLAGS=""  #TODO @mark: don't ignore existing RUSTFLAGS
#TODO @mark: extract
#TODO @mark: can this fail on warnings?

# Do not run benchmarks as it is too slow to do every time.

# Try to build the documentation.




#TODO @mark: prevent duplicate dependencies
#TODO @mark: create a --release artifact?
#TODO @mark: code coverage?
#TODO @mark: PGO?
#TODO @mark: create debian packages using cargo-deb ?


