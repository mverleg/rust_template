#!/usr/bin/env bash

##
## Run all tests, and collect coverage.
##

source "${BASH_SOURCE%/*}/general.sh"

#TODO @mark: is there a way to not apply the flags to the dependencies?

# Run tests with coverage.
# These flags are needed to give reliable coverage results (meaning a complete compile is needed).
RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" showrun cargo $CARGO_FLAGS test --profile test_coverage --workspace
