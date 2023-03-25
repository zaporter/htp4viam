{ config, pkgs, lib, ... }:
{

  # Enable the GNOME Desktop Environment.
  services.xserver.displayManager.gdm.enable = true;
  services.xserver.desktopManager.gnome.enable = true;

  # enable brightness controls
  programs.light.enable = true;

  # xdg-desktop-portal works by exposing a series of D-Bus interfaces
  # known as portals under a well-known name
  # (org.freedesktop.portal.Desktop) and object path
  # (/org/freedesktop/portal/desktop).
  # The portal interfaces include APIs for file access, opening URIs,
  # printing and others.
  services.dbus.enable = true;
  xdg.portal = {
    enable = true;
    wlr.enable = true;
    #extraPortals = [ pkgs.xdg-desktop-portal-gtk ];
  };

  fonts = {
    enableDefaultFonts = true;
    fonts = with pkgs; [
      fira-code
    ];
    fontDir.enable = true;
  };
  environment.systemPackages = with pkgs; [
    alacritty
    slack
    discord
    xdg-utils
    glib
    gnome3.adwaita-icon-theme  # default gnome cursors
    grim # screenshot 
    slurp # screenshot
    wl-clipboard # wl-copy and wl-paste from stdin/stdout
    mako # notification system
    xdg-desktop-portal-gtk #  wget
  ];
}
