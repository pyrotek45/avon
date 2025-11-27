{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  # Use beta which should have Rust 1.88+
  rust = pkgs.rust-bin.beta.latest.minimal.override {
    extensions = [ "rust-src" "rustfmt" "clippy" ];
  };
  
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in

rustPlatform.buildRustPackage {
  pname = "avon";
  version = "0.1.0";
  src = ./.;
  
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
