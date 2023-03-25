#!/usr/bin/env bash
set -eux
sudo nix-store --verify --check-contents --repair
