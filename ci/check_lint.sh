#!/usr/bin/env bash

##
## Run checks for suspicious code patterns.
##

source "${BASH_SOURCE%/*}/general.sh"

# Install tools.
if ! rustup component list | grep -q clippy
then
    showrun rustup component add clippy
fi

# Apply automatic fixes.
if [[ "$DO_FIX" = true ]]
then
    # Use '--allow-dirty' because this is already checked in general.sh (combine fmt and fix).
    showrun cargo $CARGO_FLAGS fix --clippy --workspace --all-targets --all-features --allow-dirty --allow-staged
fi

# Check code patterns.
showrun cargo $CARGO_FLAGS clippy --workspace --all-targets --all-features --tests -- -D warnings
