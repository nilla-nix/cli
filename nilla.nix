let
  pins = import ./npins;

  nilla = import pins.nilla;
in
nilla.create ({ config }: {
  config = {
    inputs = {
      fenix = {
        src = pins.fenix;
        loader = "flake";
      };
      nixpkgs = {
        src = pins.nixpkgs;

        loader = "legacy";

        settings = {
          args = {
            system = "x86_64-linux";
            overlays = [
              config.inputs.fenix.loaded.overlays.default
            ];
          };
        };
      };
    };

    packages.nilla = {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      package = { lib, pkgs, ... }:
        let
          toolchain = pkgs.fenix.complete.toolchain;
        in
        (pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
        }).buildRustPackage {
          pname = "nilla";
          version = "rust-0.0.0-pre.alpha.1";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };
    };

    shells.default = {
      systems = [ "x86_64-linux" ];

      shell = { mkShell, pkgs, ... }:
        mkShell {
          packages = [
            (pkgs.fenix.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
                "rust-analyzer"
            ])
            pkgs.bacon
            pkgs.pkg-config
          ];
        };
    };
  };
})
