#!/bin/env bash
set -e

# ----------------------
# DO NOT MODIFY

# Essentials
tool_dir="$(realpath "$(dirname "$0")")"
proton_entrypoint=$tool_dir/proton/proton
updater_entrypoint=$tool_dir/proton-updater

# Extras
extras_dir=$tool_dir/extras
discordrpc_entrypoint=$extras_dir/winediscordrpcbridge.exe
# ----------------------

echo "Checking for proton updates"
if ! "$updater_entrypoint" --install-dir-base="$tool_dir" --build="${PROTON_BUILD_ID}"; then
    if [ -f "$proton_entrypoint" ]; then
    	zenity --info --title "Update Failed" --text "Something went wrong when updating Proton to a new version, an older version will be used instead." --no-wrap --no-markup || true
        echo "Warning: $updater_entrypoint exited unsuccessfully, continuing with old proton install."
    else
    	zenity --error --title "Install Failed" --text "Something went wrong when installing Proton, aborting launch." --no-wrap --no-markup || true
        echo "FATAL: $updater_entrypoint failed and $proton_entrypoint does not exist."
        exit 1
    fi
fi

# -- EXTRAS --
mkdir -p $extras_dir
# Run the Discord RPC bridge if enabled.
if [ "$DISCORDRPC_BRIDGE" = "1" ]; then
    if [ ! -f "$discordrpc_entrypoint" ]; then
        curl --fail -L -o "$discordrpc_entrypoint" https://github.com/0e4ef622/wine-discord-ipc-bridge/releases/download/v0.0.3/winediscordipcbridge.exe
    fi
    echo "Starting Discord RPC Bridge"
    "$proton_entrypoint" run "$discordrpc_entrypoint" &
fi

# Run the game.
"$proton_entrypoint" "$@"