set -eux
zfs create \
 -o canmount=off \
 -o mountpoint=none \
 rpool/nixos
