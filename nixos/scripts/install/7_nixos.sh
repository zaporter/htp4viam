#!/usr/bin/env bash
set -eux

FLAKE_NIX="./flake.nix"
DEFAULT_NIX="./hosts/$HOSTNAME/default.nix"
HW_NIX="./hosts/$HOSTNAME/zfs_hw.nix"
MACHINE_NIX="./hosts/$HOSTNAME/machine.nix"

rm -rf /mnt/home/$USERNAME
mkdir -p /mnt/home/$USERNAME
cp -r /iso/htp4viam /mnt/home/$USERNAME/htp4viam

# dir is Read only. This fixes that
chmod -R +w /mnt/home/$USERNAME

pushd /mnt/home/$USERNAME/htp4viam/nixos

mkdir ./hosts/$HOSTNAME

cp ./templates/host.nix $DEFAULT_NIX
cp ./templates/zfs_hw.nix $HW_NIX

for i in $DISK; do
  sed -i \
  "s|PLACEHOLDER_FOR_DEV_NODE_PATH|\"${i%/*}/\"|" \
  $HW_NIX
  break
done

diskNames=""
for i in $DISK; do
  diskNames="$diskNames \"${i##*/}\""
done
tee -a $MACHINE_NIX <<EOF
{
  bootDevices = [ $diskNames ];
}
EOF

echo "Setting admin user (not root) password"
adminPwd=$(mkpasswd -m SHA-512 -s)

sed -i \
"s|PLACEHOLDER_FOR_ADMIN_PWD_HASH|\""${adminPwd}"\"|" \
$DEFAULT_NIX

# echo "Setting root password"
# rootPwd=$(mkpasswd -m SHA-512 -s)

# sed -i \
# "s|PLACEHOLDER_FOR_ROOT_PWD_HASH|\""${rootPwd}"\"|" \
# $DEFAULT_NIX


sed -i \
"s|PLACEHOLDER_USERNAME|${USERNAME}|" \
$DEFAULT_NIX


sed -i \
"s|PLACEHOLDER_HOSTNAME|${HOSTNAME}|" \
$DEFAULT_NIX


FLAKE_NIXOS_CONFIG=$(cat << EOM
      $HOSTNAME = lib.nixosSystem {
        specialArgs = {inherit inputs outputs;};
        modules = [
          ./hosts/$HOSTNAME
        ];
      };
EOM
)

sed -i "/^.*PLACEHOLDER_TOP_OF_NIXOS_CONFIGS/r /dev/stdin" $FLAKE_NIX <<<"$FLAKE_NIXOS_CONFIG"


popd


