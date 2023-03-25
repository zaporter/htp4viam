{
  config,
  pkgs,
  lib,
  inputs,
  ...
}: {
  imports = [
    ./zfs_hw.nix
    ../global
    ../users/PLACEHOLDER_USERNAME
  ];

  # I prefer to not have systems with a root pswd/acct
  # however if you want one, just uncomment this and
  # the lines in scripts/install/7_nixos.sh
  #users.users.root = {
  #  ##hash: mkpasswd -m SHA-512 -s
  #  initialHashedPassword = PLACEHOLDER_FOR_ROOT_PWD_HASH;
  #  openssh.authorizedKeys.keys = [
  #  ];
  #};

  users.users.admin = {
    ##hash: mkpasswd -m SHA-512 -s
    initialHashedPassword = PLACEHOLDER_FOR_ADMIN_PWD_HASH;
    openssh.authorizedKeys.keys = [
    ];
  };

  networking.hostName = "PLACEHOLDER_HOSTNAME";
  services.greetd.settings.default_session.user = "PLACEHOLDER_USERNAME";

  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  environment.systemPackages = with pkgs; [
  ];

  # Open ports in the firewall.
  networking.firewall.allowedTCPPorts = [80 443 8080];
  # networking.firewall.allowedUDPPorts = [ ... ];
  # Or disable the firewall altogether.
  # networking.firewall.enable = false;

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "22.11";
}
