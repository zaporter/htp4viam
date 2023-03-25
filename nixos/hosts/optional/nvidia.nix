{ config, pkgs, lib, ... }:

let
  nverStable = config.boot.kernelPackages.nvidiaPackages.stable.version;
  nverBeta = config.boot.kernelPackages.nvidiaPackages.beta.version;
  nvidiaPackage =
    if (lib.versionOlder nverBeta nverStable)
    then config.boot.kernelPackages.nvidiaPackages.stable
    else config.boot.kernelPackages.nvidiaPackages.beta;

  extraEnv = { WLR_NO_HARDWARE_CURSORS = "1"; };
in
{
  config = {
    environment.variables = extraEnv;
    environment.sessionVariables = extraEnv;

    environment.systemPackages = with pkgs; [
      glxinfo
      vulkan-tools
      lutris
      glmark2
      vulkan-headers
      vulkan-validation-layers
    ];

    hardware.nvidia.modesetting.enable = true;
    hardware.nvidia.package = nvidiaPackage;
    hardware.nvidia.powerManagement.enable = false;
    hardware.opengl.driSupport32Bit = true;
    hardware.opengl.enable = true;
    hardware.pulseaudio.support32Bit = true;
    services.xserver = {
      videoDrivers = [ "nvidia" ];
      displayManager.gdm.wayland = true;
    };
  };
}
