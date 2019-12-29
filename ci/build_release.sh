#!/usr/bin/env bash

##
## Build executables in release mode.
##

source "${BASH_SOURCE%/*}/general.sh"

# List of platforms.
# https://forge.rust-lang.org/release/platform-support.html
PLATFORMS=(
    "i686-apple-darwin"  # 32-bit OSX (10.7+, Lion+)
    "i686-pc-windows-gnu"  # 32-bit MinGW (Windows 7+)
    "i686-pc-windows-msvc"  # 32-bit MSVC (Windows 7+)
    "i686-unknown-linux-gnu"  # 32-bit Linux (2.6.18+)
    "x86_64-apple-darwin"  # 64-bit OSX (10.7+, Lion+)
    "x86_64-pc-windows-gnu"  # 64-bit MinGW (Windows 7+)
    "x86_64-pc-windows-msvc"  # 64-bit MSVC (Windows 7+)
    "x86_64-unknown-linux-gnu"  # 64-bit Linux (2.6.18+)
    "aarch64-linux-android"  # ARM64 Android
    "aarch64-unknown-linux-gnu"  # ARM64 Linux
)
#TODO @mark: wasm
#TODO @mark: native

# Create directory.
binaries_dir="$CARGO_TARGET_DIR/binaries"
mkdir -p -m 700 "$binaries_dir"

# Build for native architecture (always).
printf "platform: native"
RUSTFLAGS="-C link-arg=-s -C target-cpu=native" showrun cargo $CARGO_FLAGS build --release
executable="$(get_profile_executable release)"
showrun cp "$executable" "$binaries_dir/$(basename $executable)-native"

# Create platform binaries (if requested).
if [[ "$DO_PLATFORM_BINARIES" = true ]]
then
    for platform in ${PLATFORMS[@]}; do
        printf "platform: $platform"
        showrun rustup target add i686-apple-darwin
        platform_target_dir="$CARGO_TARGET_DIR/platform/$platform"
        RUSTFLAGS="-C link-arg=-s" showrun cargo $CARGO_FLAGS build --release --target "$platform" --target-dir "$platform_target_dir"
        executable="$(get_profile_executable platform/$platform/release)"
        #fname="$(echo "$(basename $executable)" | sed -E 's/(.*)-[^-]+/\1/')"
        showrun cp "$executable" "$binaries_dir/$(basename $executable)-$platform"
    done
fi
