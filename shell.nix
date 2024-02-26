{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
  ];
}
