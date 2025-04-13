#!/bin/env bash
set -e

if [[ "$1" == "run" ]]; then sleep 1; exit; fi

tooldir="$(realpath "$(dirname "$0")")"
proton_entrypoint=$tooldir/proton/proton

echo "-- Checking for updates --"
if ! "$tooldir/use-proton-latest" --install-dir-base="$tooldir" --repo-owner="GloriousEggroll" --repo-name="proton-ge-custom" --tarball-name="GE-Proton"; then
    if [ -f "$proton_entrypoint" ]; then
    	zenity --info --title "Update Failed" --text "Something went wrong when updating Proton-GE to a new version, an older version will be used instead." --no-wrap --no-markup || true
        echo "Warning: use-proton-latest exited unsuccessfully, continuing with old proton install."
    else
    	zenity --error --title "Install Failed" --text "Something went wrong when installing Proton-GE, aborting launch." --no-wrap --no-markup || true
        echo "FATAL: use-proton-latest failed and $proton_entrypoint does not exist."
        exit 1
    fi
fi

"$proton_entrypoint" "$@"