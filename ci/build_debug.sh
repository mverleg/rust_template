#!/usr/bin/env bash

##
## Build an executable in debug mode.
##

source "${BASH_SOURCE%/*}/general.sh"

# Build (to test, and prepare for tests).
#TODO @mark: can this fail on warnings?
clean_own_code_targets
showrun cargo $CARGOFLAGS build --workspace
