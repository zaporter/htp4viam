set -eux
pushd /mnt/home/${USERNAME}/htp4viam/nixos
git add .
nixos-install --no-root-passwd --flake .#${HOSTNAME}
useradd $USERNAME
chown -R "${USERNAME}:users" ..
popd
