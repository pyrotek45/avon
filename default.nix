{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  # Match the project shell toolchain.
  rust = pkgs.rust-bin.stable.latest.minimal.override {
    extensions = [ "rust-src" "rustfmt" "clippy" ];
  };
  
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in

rustPlatform.buildRustPackage {
  pname = "avon";
  version = "0.6.0";
  src = ./.;
  
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
