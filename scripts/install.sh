#!/bin/bash

set -e

# local paths
LOCAL_BIN_PATH_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/autosaver"
LOCAL_TMP_DIR="$(mktemp -d)"
# remote url
REMOTE_RELEASE_URL="https://github.com/daniele47/autosaver/releases/latest/download"
REMOTE_BIN_NAME=""
REMOTE_RELEASE_URL_BIN=""
# platform
PLATFORM="$(uname -s)"
ARCHITECTURE="$(uname -m)"
# state
TASKS=3

function download_url() {
    local url="$1"
    local output="$2"
    if command -v curl &>/dev/null; then
        curl -L --fail --show-error --progress-bar --output "$output" "$url"
    elif command -v wget &>/dev/null; then
        wget --no-verbose --show-progress --output-document="$output" "$url"
    else
        echo "ERROR: Neither curl nor wget found. Please install one." >&2
        return 1
    fi
}

# get platform / os dependent variables populated
case "$PLATFORM" in
Linux)
    case "$ARCHITECTURE" in
    armv7l) REMOTE_BIN_NAME="autosaver-armv7-unknown-linux-musleabihf.tar.gz" ;;
    aarch64) REMOTE_BIN_NAME="autosaver-aarch64-unknown-linux-musl.tar.gz" ;;
    x86_64) REMOTE_BIN_NAME="autosaver-x86_64-unknown-linux-musl.tar.gz" ;;
    *) echo "$PLATFORM with $ARCHITECTURE is not supported" && exit 1 ;;
    esac
    ;;
*) echo "$PLATFORM is not supported" && exit 1 ;;
esac

# delete whatever was already in the cache directory
rm -rf "$LOCAL_BIN_PATH_DIR"
mkdir -p "$LOCAL_BIN_PATH_DIR"

echo "(1/$TASKS) Downloading latest archive release..."
REMOTE_RELEASE_URL_BIN="$REMOTE_RELEASE_URL/$REMOTE_BIN_NAME"
download_url "$REMOTE_RELEASE_URL_BIN" "$LOCAL_TMP_DIR/archive.tar.gz"

echo "(2/$TASKS) Decompossing downloaded archive..."
tar -xzf "$LOCAL_TMP_DIR/archive.tar.gz" -C "$LOCAL_BIN_PATH_DIR"

echo "(3/$TASKS) Cleaning up temporary files..."
rm -rf "$LOCAL_TMP_DIR"

echo "Autosaver Installed / updated!"
