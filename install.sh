#!/bin/sh -e

if [ "$(id -u)" -eq 0 ]; then
    echo 'This script cannot be ran as the root user or with sudo. Please run it as a regular user.'
    exit 1
fi

build_name=$1
compat_dir=$2

if [ -d "$compat_dir" ]; then
  echo "$compat_dir already exists - refusing to overwrite."
  exit 1
fi

echo "Installing $build_name to $compat_dir"
mkdir -p $compat_dir

curl --fail -L -o /tmp/$build_name.tar.gz https://github.com/Blooym/proton-fusion/releases/latest/download/$build_name.tar.gz
tar -xf /tmp/$build_name.tar.gz -C $compat_dir
chmod +x $compat_dir/proton-updater

echo "Successfully installed $build_name to $compat_dir"
