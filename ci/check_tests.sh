#!/usr/bin/env bash

##
## Run all tests, and collect coverage.
## Required: libssl-dev pkg-config cmake zlib1g-dev (suggested: binutils-dev libbfd-dev)
##

source "${BASH_SOURCE%/*}/general.sh"

#TODO @mark: is there a way to not apply the flags to the dependencies?
#TODO @mark: maybe one day use grcov, but for now cannot get it to work with the limited documentation

# old tarpaulin code
# Check system support.
#if [[ $(uname -a) != *Linux* ]] || [[ $(uname -a) != *x86_64* ]]
#then
#
#    printf "\n*****\n"
#    printf "Your system: '%s'" "$(uname -a)"
#    printf "WARNING! Your system might not be supported by 'tarpaulin' (test coverage tool, supports linux on x86_64)\n" 1>&2
#    printf "WARNING! Running without coverage\n" 1>&2
#    printf "\n*****\n"
#    showrun cargo $CARGO_FLAGS test --profile test_coverage --workspace
#
#else

    # old tarpaulin code
#    # Install tool.
#    if ! hash cargo-tarpaulin 2>/dev/null
#    then
#        showrun cargo $CARGO_FLAGS install cargo-tarpaulin
#    fi
    # Install tool.
    if ! hash kcov 2>/dev/null
    then
        (
            kcov_version=37
            showrun cd "$CARGO_TARGET_DIR"
            showrun rm -rf "$CARGO_TARGET_DIR/kcov-$kcov_version"
            showrun wget https://github.com/SimonKagstrom/kcov/archive/v$kcov_version.tar.gz
            showrun tar xzf "v$kcov_version.tar.gz"
            showrun cd "$CARGO_TARGET_DIR/kcov-$kcov_version"
            showrun mkdir -p build
            showrun cd build
            showrun cmake -DCMAKE_INSTALL_PREFIX="$HOME/.local" ..
            showrun make -j8
            showrun make install
            showrun rm -rf "$CARGO_TARGET_DIR/kcov-$kcov_version" "v$kcov_version.tar.gz"
        )
    fi

    # Run tests with coverage.
    # These flags are needed to give reliable coverage results (meaning a complete compile is needed).
    # THIS IS grcov CODE, DISABLED FOR NOW
    #RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" \
    #    showrun cargo $CARGO_FLAGS test --profile test_coverage --workspace
    #showrun cargo tarpaulin --all-features --ignore-tests --line

    #TODO @mark: is RUSTFLAGS needed in this case?
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" \
        showrun cargo $CARGO_FLAGS test --profile test_coverage --workspace --no-run

    test_executable="$(find "$CARGO_TARGET_DIR/test_coverage/" -type f -executable -print0 | xargs -r -0 ls -1 -t | tail -n +1 | head -1)"
    build_hash="$(echo "$test_executable" | sed -E 's/.*-([^-]+)/\1/')"
    echo "build_hash = $build_hash"  #TODO @mark: TEMPORARY! REMOVE THIS!
    #TODO @mark: use '--report-time=plain' below when it's stabilized
    library_path="$(find target/test_coverage/ -type d -name lib -printf '%p:')"
    printf "extra library path: %s\n" "$library_path"
    export LD_LIBRARY_PATH="${library_path}${LD_LIBRARY_PATH}"
    showrun kcov --exclude-pattern=/.cargo target/kcov "$test_executable" --test

    #TODO @mark: RUSTFLAGS should be reset at this point

# End of system support check
#fi
