{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  packages = with pkgs; [
    rustup
  ];
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
