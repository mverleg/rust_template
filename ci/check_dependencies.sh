#!/usr/bin/env bash

##
## Run checks on dependencies (duplicates, outdated, licenses...).
##
## Note that things can still get outdated *after* release.
##

source "${BASH_SOURCE%/*}/general.sh"

# Install tools.
if ! hash cargo-build-deps 2>/dev/null
then
    showrun cargo $CARGO_FLAGS install cargo-build-deps
fi
if ! hash cargo-audit 2>/dev/null
then
    showrun cargo $CARGO_FLAGS install cargo-audit
fi
if ! hash cargo-outdated 2>/dev/null
then
    showrun cargo $CARGO_FLAGS install --force --git https://github.com/kbknapp/cargo-outdated
fi
if ! hash cargo-deny 2>/dev/null
then
    showrun cargo $CARGO_FLAGS install cargo-deny
fi
if ! hash cargo-tree 2>/dev/null
then
    showrun cargo $CARGO_FLAGS install cargo-tree
fi

# Check crate dependencies.
if [[ ! -d "$CARGO_TARGET_DIR" ]] ||
    [[ ! -f "$CARGO_TARGET_DIR/dependencies-checked" ]] ||
    [[ $(($(date +%s) - $(stat -c '%Y' "$CARGO_TARGET_DIR/dependencies-checked"))) -gt 3600 ]]
then
    find . -maxdepth 5 -type f -name 'Cargo.toml' -print0 |
    while IFS= read -r -d '' cargo_file
    do
        if [[ ! -f "$CARGO_TARGET_DIR/dependencies-checked" ]] ||
            [[ $(($(stat -c '%Y' "$cargo_file") - $(stat -c '%Y' "$CARGO_TARGET_DIR/dependencies-checked"))) -gt 0 ]]
        then

            # Run various checks on the dependencies.
            showrun cargo $CARGO_FLAGS audit --deny-warnings
            showrun cargo $CARGO_FLAGS outdated --exit-code 1
            showrun cargo $CARGO_FLAGS deny check licenses
            showrun cargo $CARGO_FLAGS deny check advisories
            showrun cargo $CARGO_FLAGS deny check bans  # mostly for checking duplicates

            # Disable these checks for a while.
            if [[ -d "$CARGO_TARGET_DIR" ]]
            then
                touch "$CARGO_TARGET_DIR/dependencies-checked"
            fi

            # This needs to run once if any Cargo.toml is outdated, so break the loop.
            break
        fi
    done
else
    printf "Skipping dependency checks, because they were already done within the last hour\n"
fi

# Write information about dependencies.
cargo $CARGO_FLAGS tree > "$REPORT_DIR/dependencies.txt"
