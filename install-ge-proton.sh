#!/bin/sh -e

if [ "$(id -u)" -eq 0 ]; then
    echo 'This script cannot be ran as the root user or with sudo. Please run it as a regular user.'
    exit 1
fi

compat_dir=$1
if [ -d "$compat_dir" ]; then
  echo "$compat_dir already exists - refusing to overwrite."
  exit 1
fi

echo "Installing GE-Proton-Fusion to $compat_dir"
mkdir -p $compat_dir

curl --fail -L -o /tmp/GE-Proton-Fusion.tar.gz https://github.com/Blooym/proton-fusion/releases/latest/download/GE-Proton-Fusion.tar.gz
tar -xf /tmp/GE-Proton-Fusion.tar.gz -C $compat_dir
chmod +x $compat_dir/proton-updater

echo "Successfully installed GE-Proton-Fusion to $compat_dir"