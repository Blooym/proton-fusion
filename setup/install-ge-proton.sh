#!/bin/sh -e

if [ "$(id -u)" -eq 0 ]; then
    echo 'This script cannot be ran as the root user or with sudo. Please run it as a regular user.'
    exit 1
fi

compat_dir=$1
if [ -d "$compat_dir" ]; then
  echo "$$compat_dir already exists - refusing to overwrite."
fi

echo "Installing GE-Proton-Latest to $compat_dir"
mkdir -p $compat_dir

curl --fail -L -o /tmp/GE-Proton.tar.gz https://github.com/Blooym/use-proton-latest/releases/latest/download/GE-Proton.tar.gz
tar -xf /tmp/GE-Proton.tar.gz -C $compat_dir
chmod +x $compat_dir/use-proton-latest

echo "Successfully installed GE-Proton-Latest to $compat_dir"