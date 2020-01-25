#!/usr/bin/env bash

##
## Run checks for code style.
##

source "${BASH_SOURCE%/*}/general.sh"

# Install tools.
if ! rustup component list | grep -q rustfmt
then
    showrun rustup component add rustfmt
fi

# Apply automatic fixes.
if [[ "$DO_FIX" = true ]]
then
    showrun cargo $CARGO_FLAGS fmt
fi

# Check code style.
clean_own_code_targets
showrun cargo $CARGO_FLAGS fmt -- --check

printf "✓ checking style ready\n"
