{pkgs, inputs,lib, config, ...}:
{
  nix = {
    settings = {
      trusted-users = ["root" "@wheel"];
      auto-optimise-store = lib.mkDefault true;
      experimental-features = ["nix-command" "flakes" "repl-flake"];
      warn-dirty = true;
      system-features = ["kvm"];
    };
    gc = {
      automatic = true;
      dates = "weekly";
    };
  };
}
