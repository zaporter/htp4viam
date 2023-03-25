{config,pkgs, lib, inputs, ...}:

# DRAWN FROM
# https://github.com/NixOS/nixpkgs/tree/master/nixos/modules/installer/cd-dvd/
# and 
# https://hoverbear.org/blog/nix-flake-live-media/

{
  imports = [
    ../global
  ];

  nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";

  networking.hostName = "iso"; # Define your hostname.
  # Disable wireless because we use networkmanager
  networking.wireless.enable = false;
  systemd.services.sshd.wantedBy = lib.mkOverride 40 [ "multi-user.target" ];

  users.users.root = {
    password = "htp4viam";
  };

  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  environment.systemPackages = with pkgs; [
  ];

  # Open ports in the firewall.
  networking.firewall.allowedTCPPorts = [ 8000 ];
  # networking.firewall.allowedUDPPorts = [ ... ];
  # Or disable the firewall altogether.
  # networking.firewall.enable = false;

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "22.11"; # Did you read the comment?
}
