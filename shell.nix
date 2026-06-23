{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    gnumake
    cargo
    rustc
    clippy
    rustfmt
  ];
}
