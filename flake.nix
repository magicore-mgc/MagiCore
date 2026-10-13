# Nix development environment and package for the current Cargo workspace.
{
  description = "MagiCore development environment and CLI package";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        rust-platform = pkgs.makeRustPlatform {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        workspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in {
        devShells.default = pkgs.mkShell {
          name = "magicore-dev";
          packages = with pkgs; [
            rust-toolchain
            go
            nodejs_24
            protobuf
            pkg-config
            openssl
            zlib
          ];

          shellHook = ''
            echo "MagiCore Rust workspace ready (minimum Rust 1.94)."
            echo "Available commands: cargo check --workspace; cargo test --workspace"
          '';
        };

        packages.default = rust-platform.buildRustPackage {
          pname = "mgc";
          version = workspace.workspace.package.version;
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          buildAndTestSubdir = "cli";
          nativeBuildInputs = with pkgs; [ go pkg-config ];
          buildInputs = with pkgs; [ openssl zlib ];
          # Workspace tests run in the regular CI matrix with their toolchains.
          doCheck = false;

          meta = {
            description = "MagiCore package manager CLI";
            mainProgram = "mgc";
            platforms = pkgs.lib.platforms.unix;
          };
        };
      });
}
