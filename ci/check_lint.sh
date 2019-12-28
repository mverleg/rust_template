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
    showrun cargo $CARGO_FLAGS fix --clippy --workspace --all-targets --all-features
fi

# Check code patterns.
showrun cargo $CARGO_FLAGS clippy --workspace --all-targets --all-features --tests -- -D warnings
