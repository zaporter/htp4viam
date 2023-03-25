#!/usr/bin/env bash
set -eu

# Get list of all drives on the system
drives=( $(lsblk -rpo "name,type,mountpoint" | awk '$2=="disk"{print $1}') )
drive_sizes=( $(lsblk -rpo "size,type,mountpoint" | awk '$2=="disk"{print $1}') )

# Print numbered list of drives and their sizes
for i in "${!drives[@]}"; do
    echo "$i: ${drives[$i]} (${drive_sizes[$i]})"
done

# Prompt user to select a drive to install the iso on
echo "Enter the number corresponding to the drive you want to install the iso on:"
read selected_drive

# Prompt user to confirm if this is the right drive
echo "You have selected drive ${drives[$selected_drive]}. Is this correct? (y/N)"
read confirmation
if [ "$confirmation" != "y" ]; then
    echo "Please select the correct drive and run the script again."
    exit 1
fi

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
pushd $SCRIPTPATH
cd ..
# Build and install the iso
nix build --impure .#nixosConfigurations.iso.config.system.build.isoImage
iso_loc=( $(ls result/iso/*.iso) )
sudo dd bs=4M if=${iso_loc} of=${drives[$selected_drive]} status=progress oflag=sync
popd
