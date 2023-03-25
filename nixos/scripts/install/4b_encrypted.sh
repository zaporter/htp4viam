set -eux
zfs create \
 -o canmount=off \
 -o mountpoint=none \
 -o encryption=on \
 -o keylocation=prompt \
 -o keyformat=passphrase \
 rpool/nixos
