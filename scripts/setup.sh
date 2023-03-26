#!/bin/bash
set -eux

# --------- Install SSH Key ----------------
#
echo "Installing public ssh key"

# Will be replaced with the actual key
PUBLIC_KEY="PLACEHOLDER_PUBLIC_KEY"

# Ensure the .ssh directory exists and set proper permissions
mkdir -p ~/.ssh
chmod 700 ~/.ssh

# Append the public key to authorized_keys and set proper permissions
echo "$PUBLIC_KEY" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys

echo "Public key installed successfully."

