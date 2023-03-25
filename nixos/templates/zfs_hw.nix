{ config, lib, pkgs, modulesPath, ... }:

let
  zfsRoot.partitionScheme = {
    biosBoot = "-part5";
    efiBoot = "-part1";
    swap = "-part4";
    bootPool = "-part2";
    rootPool = "-part3";
  };
  zfsRoot.devNodes = PLACEHOLDER_FOR_DEV_NODE_PATH; # MUST have trailing slash! /dev/disk/by-id/
  zfsRoot.bootDevices = (import ./machine.nix).bootDevices;
  zfsRoot.mirroredEfi = "/boot/efis/";

in {
  # adjust according to your platform, such as
  imports = [
    # (modulesPath + "/profiles/qemu-guest.nix")
    # (modulesPath + "/profiles/all-hardware.nix")
    (modulesPath + "/installer/scan/not-detected.nix")
  ];
  systemd.services.zfs-mount.enable = false;

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "22.11"; # Did you read the comment?

  boot.initrd.availableKernelModules = [
    "ahci"
    "xhci_pci"
    "virtio_pci"
    "virtio_blk"
    "ehci_pci"
    "nvme"
    "uas"
    "sd_mod"
    "sr_mod"
    "sdhci_pci"
    # custom
    "usb_storage"
    "usbhid"
    "rtsx_pci_sdmmc"
    "thunderbolt"
  ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [ "kvm-intel" "kvm-amd" ];
  boot.extraModulePackages = [ ];

  fileSystems = {
    "/" = {
      device = "rpool/nixos/root";
      fsType = "zfs";
      options = [ "X-mount.mkdir" ];
    };

    "/home" = {
      device = "rpool/nixos/home";
      fsType = "zfs";
      options = [ "X-mount.mkdir" ];
    };

    "/var/lib" = {
      device = "rpool/nixos/var/lib";
      fsType = "zfs";
      options = [ "X-mount.mkdir" ];
    };

    "/var/log" = {
      device = "rpool/nixos/var/log";
      fsType = "zfs";
      options = [ "X-mount.mkdir" ];
    };

    "/boot" = {
      device = "bpool/nixos/root";
      fsType = "zfs";
      options = [ "X-mount.mkdir" ];
    };
  } // (builtins.listToAttrs (map (diskName: {
    name = zfsRoot.mirroredEfi + diskName + zfsRoot.partitionScheme.efiBoot;
    value = {
      device = zfsRoot.devNodes + diskName + zfsRoot.partitionScheme.efiBoot;
      fsType = "vfat";
      options = [
        "x-systemd.idle-timeout=1min"
        "x-systemd.automount"
        "noauto"
        "nofail"
      ];
    };
  }) zfsRoot.bootDevices));

  swapDevices = (map (diskName: {
    device = zfsRoot.devNodes + diskName + zfsRoot.partitionScheme.swap;
    discardPolicy = "both";
    randomEncryption = {
      enable = true;
      allowDiscards = true;
    };
  }) zfsRoot.bootDevices);

  # Enables DHCP on each ethernet and wireless interface. In case of scripted networking
  # (the default) this is the recommended approach. When using systemd-networkd it's
  # still possible to use this option, but it's recommended to use it in conjunction
  # with explicit per-interface declarations with `networking.interfaces.<interface>.useDHCP`.
  networking.useDHCP = lib.mkDefault true;
  # networking.networkmanager.enable = true;
  # networking.interfaces.enp1s0.useDHCP = lib.mkDefault true;

  nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
  hardware.cpu.intel.updateMicrocode =
    lib.mkDefault config.hardware.enableRedistributableFirmware;
  hardware.cpu.amd.updateMicrocode =
    lib.mkDefault config.hardware.enableRedistributableFirmware;

  boot.supportedFilesystems = [ "zfs" ];
  networking.hostId = "abcd1234";
  boot.kernelPackages = config.boot.zfs.package.latestCompatibleLinuxPackages;
  boot.loader.efi.efiSysMountPoint = with builtins;
    (zfsRoot.mirroredEfi + (head zfsRoot.bootDevices) + zfsRoot.partitionScheme.efiBoot);
  boot.zfs.devNodes = zfsRoot.devNodes;
  boot.loader.efi.canTouchEfiVariables = false;
  boot.loader.generationsDir.copyKernels = true;
  boot.loader.grub.efiInstallAsRemovable = true;
  boot.loader.grub.enable = true;
  boot.loader.grub.version = 2;
  boot.loader.grub.copyKernels = true;
  boot.loader.grub.efiSupport = true;
  boot.loader.grub.zfsSupport = true;
  boot.loader.grub.extraInstallCommands = with builtins;
    (toString (map (diskName:
      "cp -r " + config.boot.loader.efi.efiSysMountPoint + "/EFI" + " "
      + zfsRoot.mirroredEfi + diskName + zfsRoot.partitionScheme.efiBoot + "\n")
      (tail zfsRoot.bootDevices)));
  boot.loader.grub.devices =
    (map (diskName: zfsRoot.devNodes + diskName) zfsRoot.bootDevices);
}

