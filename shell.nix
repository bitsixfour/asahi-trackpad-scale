{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.openssl
    pkgs.pkg-config
    pkgs.dbus
  ];
}
