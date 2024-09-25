{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  src = pkgs.lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;
  cargoLock.outputHashes = {
    "anyrun-interface-0.1.0" = "sha256-ZhSA0e45UxiOAjEVqkym/aULh0Dt+KHJLNda7bjx9UI=";
  };
  
  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  buildInputs = [
    pkgs.openssl
  ];

}

