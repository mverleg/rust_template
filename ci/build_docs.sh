#!/usr/bin/env bash

##
## Build documentation.
##

source "${BASH_SOURCE%/*}/general.sh"

showrun cargo $CARGO_FLAGS doc --no-deps --all-features
