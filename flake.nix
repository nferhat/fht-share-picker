{
  description = "A dynamic tiling Wayland compositor.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {self, ...}:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"]; # TODO: aarch64? though I don't use it.
      perSystem = {
        self',
        pkgs,
        inputs',
        ...
      }: let
      in {
        # NOTE: This is for the Nix code formatter!!
        formatter = pkgs.alejandra;

        packages = let
          fht-share-picker-package = {
            lib,
            glib,
            gtk4,
            libadwaita,
            libxkbcommon,
            pkg-config,
            rustPlatform,
          }:
            rustPlatform.buildRustPackage {
              pname = "fht-share-picker";
              version = self.shortRev or self.dirtyShortRev or "unknown";
              src = ./.;

              cargoLock = {
                # sctk is pulled from git
                allowBuiltinFetchGit = true;
                lockFile = ./Cargo.lock;
              };

              strictDeps = true;
              # NOTE: We need glib in nativeBuildInputs for glib-compile-resources
              nativeBuildInputs = [rustPlatform.bindgenHook pkg-config glib];
              buildInputs = [glib gtk4 libadwaita libxkbcommon];

              meta = {
                homepage = "https://github.com/nferht/fht-share-picker";
                license = lib.licenses.gpl3Only;
                mainProgram = "fht-share-picker";
                platforms = lib.platforms.linux;
              };
            };
        in rec {
          fht-share-picker = pkgs.callPackage fht-share-picker-package {};
          default = fht-share-picker;
        };

        devShells.default = let
          rust-bin = inputs.rust-overlay.lib.mkRustBin {} pkgs;
        in
          pkgs.mkShell {
            packages = with pkgs; [
              # For developement purposes, a nightly toolchain is preferred.
              # We use nightly cargo for formatting, though compiling is limited to
              # whatever is specified inside ./rust-toolchain.toml
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.default.override {
                  extensions = ["rust-analyzer" "rust-src"];
                }))

              pkgs.alejandra # for formatting this flake if needed

              # The base libraries needed for a gtk application
              # libxkbcommon for sctk since I use it to bind protocols
              glib
              gtk4
              libadwaita
              libxkbcommon
            ];

            nativeBuildInputs = with pkgs; [rustPlatform.bindgenHook pkg-config];
          };
      };
    };
}
