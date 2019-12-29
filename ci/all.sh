#!/usr/bin/env bash

d="${BASH_SOURCE%/*}"
source "$d/general.sh"
source "$d/check_dependencies.sh"
source "$d/build_dependencies.sh"
source "$d/build_debug.sh"
source "$d/check_style.sh"
source "$d/check_lint.sh"
source "$d/check_tests.sh"
source "$d/build_docs.sh"

if [[ "$DO_FIX" = true ]] && [[ -n "$(git status --porcelain)" ]]
then
    printf "Automatic changes were made, do not forget to commit them!\n"
fi

#TODO @mark: create a --release artifact?
#TODO @mark: PGO?
#TODO @mark: create debian packages using cargo-deb ?

