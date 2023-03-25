{
  description = "NixOS config for htp4viam";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.11";

    nixneovim.url = "github:nixneovim/nixneovim";
    nixneovim.inputs.nixpkgs.follows = "nixpkgs";

    nixos.url = "github:nixos/nixpkgs/nixos-22.11";
  };

  outputs = {
    self,
    nixos,
    nixpkgs,
    nixneovim,
    ...
  } @ inputs: let
    inherit (self) outputs;
    system = "x86_64-linux";
    lib = nixpkgs.lib;
    inherit
      (import ./overlays {inherit inputs outputs;})
      overlays
      ;
    pkgs = import nixpkgs {
      inherit system overlays;
      config = {allowUnfree = true;};
    };
  in {
    templates = import ./templates;
    devShells = import ./shell.nix {inherit pkgs;};

    nixosConfigurations = {
      # PLACEHOLDER_TOP_OF_NIXOS_CONFIGS

      iso = lib.nixosSystem {
        specialArgs = {inherit inputs outputs;};
        modules = [
          ./hosts/iso
          "${nixos}/nixos/modules/installer/cd-dvd/installation-cd-minimal.nix"
          ({pkgs, ...}: {
            isoImage.contents = [
              {
                source = ./..;
                target = "/htp4viam";
              }
              {
                source = /home/zack/personal/htp4viam/.git;
                target = "/htp4viam/.git";
              }
            ];
          })
        ];
      };
    };
  };
}
