#!/bin/env bash
set -e

if [[ "$1" == "run" ]]; then sleep 1; exit; fi

tooldir="$(realpath "$(dirname "$0")")"
proton_entrypoint=$tooldir/proton/proton

echo "-- Checking for updates --"
if ! "$tooldir/use-proton-latest" --install-dir-base="$tooldir" --repo-owner="GloriousEggroll" --repo-name="proton-ge-custom" --tarball-name="GE-Proton"; then
    if [ -f "$proton_entrypoint" ]; then
        echo "Warning: use-proton-latest exited unsuccessfully, continuing with old proton install."
    else
        echo "FATAL: use-proton-latest failed and $proton_entrypoint does not exist."
        exit 1
    fi
fi

echo "-- Starting proton --"
if [ ! -f "$tooldir/proton/proton" ]; then
    echo "FATAL: $tooldir/proton/proton not found."
    exit 1
fi

"$proton_entrypoint" "$@"