#!/bin/bash

set -e

CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/autosaver"

TASKS=1
echo "(1/$TASKS) Deleting cache directory with binary and completions..."
rm -rf "$CACHE_DIR"

echo "Autosaver uninstalled! Remember to remove source commands in the various shells!"
