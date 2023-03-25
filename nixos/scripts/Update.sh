#!/usr/bin/env bash
set -eux
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
pushd $SCRIPTPATH
cd ..
nix flake update
popd
