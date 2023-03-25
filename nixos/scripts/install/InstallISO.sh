#!/usr/bin/env bash
# @author zaporter
#
#
set -eu

if [ `whoami` != "root" ]; then
    echo "The install script must be run as root"
    exit 1
fi

if ping -c 1 9.9.9.9 &> /dev/null
then
  echo "Internet connection successfully verified"
else
  echo "This install script requires an internet connection"
  exit 1
fi


echo "WARNING!!"
echo "This script will completely wipe your hard drive and install a new operating system. Are you sure you want to continue? (y/N)"
read confirmation
if [ "$confirmation" != "y" ]; then
    exit 1
fi

# Get list of all drives on the system
drives=( $(find /dev/disk/by-id/ | grep -v "part" | sed 's|.*/||') )
drive_sizes=( $(for drive in "${drives[@]}"; do echo $(($(blockdev --getsize64 /dev/disk/by-id/$drive) / 1024 / 1024 / 1024 ))"G"; done) )

# Print numbered list of drives and their sizes
for i in "${!drives[@]}"; do
    echo "$i: ${drives[$i]} (${drive_sizes[$i]})"
done

# Prompt user to select one or more drives to install the iso on
echo "Enter the number(s) corresponding to the drive(s) you want to install the iso on (space separated) (ex: 0 1 4):"
read -a selected_drive_nums

SELECTED_DRIVES=()
for i in "${selected_drive_nums[@]}"; do
    SELECTED_DRIVES+=("/dev/disk/by-id/${drives[$i]}")
done
SELECTED_DRIVES="${SELECTED_DRIVES[*]}"

# Prompt user to confirm if this is the right drive
echo "You have selected drives: (${SELECTED_DRIVES}). Is this correct? (y/N)"
read confirmation
if [ "$confirmation" != "y" ]; then
    echo "Please select the correct drives and run the script again."
    exit 1
fi

export DISK="$SELECTED_DRIVES"
export USERNAME="admin"
export SYSTEM="x86_64-linux"
export INST_PARTSIZE_SWAP=4
export INST_PARTSIZE_RPOOL=

# Prompt user to select a drive to install the iso on
echo "Enter the desired hostname of the device:"
read HOSTNAME
export HOSTNAME

function section() {
  local section_name="$1"
  echo -e "\n------------------------------------------------"
  echo -e "          ${section_name^^}"
  echo -e "------------------------------------------------\n"
}

section "Starting installation"
pushd /iso/htp4viam/nixos/scripts/install
section "0. Wipe"
./0_wipe.sh
section "1. Partition"
./1_partition.sh
section "2. Boot pool"
./2_boot_pool.sh
section "3. Root pool"
./3_root_pool.sh
section "4. Setup drive (encrypted/unencrypted)"
./4a_unencrypted.sh
section "5. Create datasets"
./5_create_datasets.sh
section "6. Create EFI system patition"
./6_create_efi_system_partition.sh
section "7. Setup NixOS"
./7_nixos.sh
section "8. Install NixOS"
./8_install_nixos.sh
section "9. Finish"
./9_finish.sh
popd

section "Installation finished"
echo "Shutdown this device, remove the install medium, and turn back on the device."
