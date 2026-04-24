#!/bin/bash

export PATH+=":${CARGO_HOME:-$HOME/.cargo}/bin"

if ! command -v cross &>/dev/null; then
    echo "you need to install cross via $(cargo install cross)"
    exit 1
fi

SCRIPT_PATH="$(realpath "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$SCRIPT_PATH")"
BUILD_DIR="$SCRIPT_DIR/builds"

TARGETS=(
    x86_64-unknown-linux-musl
    armv7-unknown-linux-musleabihf
)

for target in "${TARGETS[@]}"; do
    echo -e "\e[1;37mCOMPILING FOR '$target'...\e[m"
    cross build --target "$target" --release
    echo -e "\e[1;37mCOPYING COMPILED FILE INTO 'build' directory...\e[m"
    mkdir -p "$BUILD_DIR"
    cp "$SCRIPT_DIR/.target/$target/release/dotfiles-rust" "$BUILD_DIR/dotfiles-rust-$target"
    echo "----------------------------------------"
done
