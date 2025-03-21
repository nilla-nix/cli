let
  pins = import ./npins;

  nilla = import pins.nilla;
in
nilla.create ({ config }: {
  config = {
    inputs = {
      fenix = {
        src = pins.fenix;
      };

      nixpkgs = {
        src = pins.nixpkgs;

        settings = {
          overlays = [
            config.inputs.fenix.loaded.overlays.default
          ];
        };
      };
    };

    packages.nilla = {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      package = { fenix, makeRustPlatform, ... }:
        let
          toolchain = fenix.complete.toolchain;

          platform = makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        platform.buildRustPackage {
          pname = "nilla";
          version = "rust-0.0.0-pre.alpha.1";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };
    };

    shells.default = {
      systems = [ "x86_64-linux" ];

      shell = { mkShell, fenix, bacon, pkg-config, ... }:
        mkShell {
          packages = [
            (fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
              "rust-analyzer"
            ])
            bacon
            pkg-config
          ];
        };
    };
  };
})
