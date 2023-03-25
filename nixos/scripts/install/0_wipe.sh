set -eux
wipefs -a `find /dev/disk/by-id | grep "$DISK"`
