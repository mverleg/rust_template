#!/usr/bin/env bash

##
## Build only the dependencies.
##

source "${BASH_SOURCE%/*}/general.sh"

#TODO @mark: share dependencies https://stackoverflow.com/questions/59511731/share-compiled-dependencies-between-dev-and-release-builds

showrun cargo $CARGO_FLAGS build --workspace
clean_own_code_targets


