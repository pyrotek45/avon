{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  # Use stable Rust
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rustfmt" "clippy" ];
    targets = [ "x86_64-unknown-linux-gnu" ];
  };
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    cargo
    rustc
    rustfmt
    clippy
    pkg-config
    openssl
  ];

  shellHook = ''
    echo "Avon development environment loaded"
    echo "Rust version: $(rustc --version)"
  '';
}
