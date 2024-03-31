{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  buildInputs = [
    just
    # rustup
    nodePackages.prettier
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}