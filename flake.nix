{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          nonRustDeps = [
            pkgs.pkg-config
            pkgs.stdenv.cc.cc
            pkgs.zlib
            pkgs.openssl
            pkgs.protobuf
          ];
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = [
              pkgs.rustc
              pkgs.cargo
              pkgs.clippy
              pkgs.cargo-watch
              pkgs.rust-analyzer
              pkgs.rustPlatform.rustcSrc
              (pkgs.python3.withPackages (python-pkgs: [
                python-pkgs.numpy
                python-pkgs.pyarrow
              ]))
              pkgs.maturin
            ];
          };
          NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.stdenv.cc.cc
              pkgs.zlib
              pkgs.openssl
              pkgs.protobuf
            ];
        in
        {
          # Rust package
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inherit NIX_LD_LIBRARY_PATH;
            inputsFrom = [
              config.treefmt.build.devShell
            ];
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}

              echo
              echo "🍎🍎 Run 'just <recipe>' to get started"
              just
            '';
            buildInputs = nonRustDeps;
            nativeBuildInputs = with pkgs; [
              just
              rust-toolchain
              (pkgs.hiPrio pkgs.bashInteractive) # needed so it doesn't mangle terminal in vscode
              python311
              pre-commit
            ];
            # RUST_BACKTRACE = 1;
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    };
}
