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

    packages.default = config.packages.nilla-cli;
    packages.nilla-cli = {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      package = { fenix, makeRustPlatform, importTOML, ... }:
        let
          toolchain = fenix.complete.toolchain;

          manifest = (importTOML ./Cargo.toml).package;

          platform = makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        platform.buildRustPackage {
          pname = manifest.name;
          version = "rust-${manifest.version}";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };
    };

    shells.default = config.shells.nilla-cli;
    shells.nilla-cli = {
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
