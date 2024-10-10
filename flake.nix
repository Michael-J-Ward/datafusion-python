{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
      perSystem = { config, self', lib, system, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
            config = { };
          };
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          nonRustDeps = [
            pkgs.pkg-config
            pkgs.stdenv.cc.cc
            pkgs.zlib
            pkgs.openssl
            pkgs.protobuf
          ];
          rust-toolchain = python: pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = [
              (pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml)
              pkgs.clippy
              pkgs.cargo-watch
              pkgs.rust-analyzer
              pkgs.rustPlatform.rustcSrc
              (python.withPackages (python-pkgs: [
                python-pkgs.numpy
                python-pkgs.pyarrow
                python-pkgs.pandas
                python-pkgs.polars
                # python-pkgs.PyGithub
              ]))
              pkgs.maturin
              # used for cargo llvm-cov coverage
              # pkgs.grcov
              # pkgs.cargo-binutils
              # pkgs.cargo-llvm-cov
              # pkgs.rustc.llvmPackages.llvm
            ];
          };
          NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc
            pkgs.zlib
            pkgs.openssl
            pkgs.protobuf
          ];


          # see: https://gist.github.com/ChadSki/926e5633961c9b48131eabd32e57adb2
          # Conda installs it's packages and environments under this directory
          installationPath = "~/.conda";
          miniconda-version = "4.3.11";

          # Downloaded Miniconda installer
          minicondaScript = pkgs.stdenv.mkDerivation rec {
            name = "miniconda-${miniconda-version}";
            version = miniconda-version;
            src = pkgs.fetchurl {
              url = "https://repo.continuum.io/miniconda/Miniconda3-${miniconda-version}-Linux-x86_64.sh";
              sha256 = "1f2g8x1nh8xwcdh09xcra8vd15xqb5crjmpvmc2xza3ggg771zmr";
            };
            # Nothing to unpack.
            unpackPhase = "true";
            # Rename the file so it's easier to use. The file needs to have .sh ending
            # because the installation script does some checks based on that assumption.
            # However, don't add it under $out/bin/ becase we don't really want to use
            # it within our environment. It is called by "conda-install" defined below.
            installPhase = ''
              mkdir -p $out
              cp $src $out/miniconda.sh
            '';
            # Add executable mode here after the fixup phase so that no patching will be
            # done by nix because we want to use this miniconda installer in the FHS
            # user env.
            fixupPhase = ''
              chmod +x $out/miniconda.sh
            '';
          };

          # Wrap miniconda installer so that it is non-interactive and installs into the
          # path specified by installationPath
          conda = pkgs.runCommand "conda-install"
            { buildInputs = [ pkgs.makeWrapper minicondaScript ]; }
            ''
              mkdir -p $out/bin
              makeWrapper                            \
                ${minicondaScript}/miniconda.sh      \
                $out/bin/conda-install               \
                --add-flags "-p ${installationPath}" \
                --add-flags "-b"
            '';

          mkPythonDevShell = pythonPkg: pkgs.mkShell {
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
              (rust-toolchain pythonPkg)
              (pkgs.hiPrio pkgs.bashInteractive) # needed so it doesn't mangle terminal in vscode
              pythonPkg
              pre-commit
            ];
            # RUST_BACKTRACE = 1;
          };
        in
        {
          # Rust package
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          devShells = {
            default = mkPythonDevShell pkgs.python311;
            py310 = mkPythonDevShell pkgs.python310;
            py312 = mkPythonDevShell pkgs.python312;
            py313 = mkPythonDevShell pkgs.python313;
            py37 = mkPythonDevShell pkgs.python37;
          };

          # NIXPKGS_ALLOW_UNFREE=1 nix develop '.#conda' --impure
          # conda install -c conda-forge boa conda-verify
          # NOTE: conda env create fails probably because I am not on  nvidia
          # conda env create -f ./conda/environments/datafusion-dev.yaml -n datafusion-dev
          # > ResolvePackageNotFound: cudf
          devShells.conda = (pkgs.buildFHSUserEnv {
            name = "conda";
            targetPkgs = pkgs: (
              with pkgs; [
                autoconf
                binutils
                conda
                cudatoolkit
                curl
                freeglut
                gcc11
                git
                gitRepo
                gnumake
                gnupg
                gperf
                libGLU
                libGL
                libselinux
                linuxPackages.nvidia_x11
                m4
                ncurses5
                procps
                stdenv.cc
                unzip
                util-linux
                wget
                xorg.libICE
                xorg.libSM
                xorg.libX11
                xorg.libXext
                xorg.libXi
                xorg.libXmu
                xorg.libXrandr
                xorg.libXrender
                xorg.libXv
                zlib
              ]
            );
            profile = ''
              # cuda
              export CUDA_PATH=${pkgs.cudatoolkit}
              # export LD_LIBRARY_PATH=${pkgs.linuxPackages.nvidia_x11}/lib
              export EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib"
              export EXTRA_CCFLAGS="-I/usr/include"
              # conda
              export PATH=${installationPath}/bin:$PATH
              # Paths for gcc if compiling some C sources with pip
              export NIX_CFLAGS_COMPILE="-I${installationPath}/include"
              export NIX_CFLAGS_LINK="-L${installationPath}lib"
              # Some other required environment variables
              export FONTCONFIG_FILE=/etc/fonts/fonts.conf
              export QTCOMPOSE=${pkgs.xorg.libX11}/share/X11/locale
            '';
          }).env;

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
