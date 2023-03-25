{
  lib,
  config,
  pkgs,
  inputs,
  outputs,
  ...
}: {
  imports = [
    ./locale.nix
    ./nix.nix
  ];

  programs.zsh.enable = true;

  # local device discovery
  services.avahi = {
    enable = true;
    publish = {
      enable = true;
      addresses = true;
      workstation = true;
    };
    nssmdns = true;
  };

  # Enable networking
  networking.networkmanager.enable = true;

  # Enable Polkit for system-wide privileges
  security.polkit.enable = true;

  programs.ssh.startAgent = true;
  services.nscd.enable = true;

  nixpkgs = {
    config = {
      allowUnfree = true;
    };
  };

  virtualisation.docker.enable = true;
  # zfs does not support swap. This will fail without this
  boot.kernelParams = ["nohibernate"];

  # Enable the OpenSSH daemon.
  services.openssh.enable = true;
  environment.systemPackages = with pkgs; [
    git
    neovim
    curl
    wget
    zsh
    os-prober
  ];
}
