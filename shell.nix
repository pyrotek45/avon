{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  # Use stable Rust with security extensions
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rustfmt" "clippy" ];
    targets = [ "x86_64-unknown-linux-gnu" ];
  };
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rust
    cargo
    rustc
    rustfmt
    clippy
    
    # Security and dependency auditing
    cargo-audit    # Audit Cargo.lock for known security vulnerabilities
    cargo-deny     # Deny malicious/unmaintained dependencies
    
    # Required dependencies
    pkg-config
    openssl
    
    # Helpful tools for development
    git            # Version control
    
    # Benchmarking
    hyperfine      # Command-line benchmarking tool
  ];

  shellHook = ''
    echo "=========================================="
    echo "Avon development environment loaded"
    echo "Rust version: $(rustc --version)"
    echo ""
    echo "Security tools available:"
    echo "  • cargo audit     - Check for known vulnerabilities"
    echo "  • cargo deny      - Audit dependencies for issues"
    echo "  • cargo clippy    - Lint for code quality & security"
    echo ""
    echo "Run these before committing code:"
    echo "  cargo clippy --all-targets"
    echo "  cargo audit"
    echo "=========================================="
  '';
}
