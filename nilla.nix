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
            config.inputs.fenix.result.overlays.default
          ];
        };
      };
    };

    packages.default = config.packages.nilla-cli;
    packages.nilla-cli = {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

      package = { fenix, makeRustPlatform, lib, installShellFiles, ... }:
        let
          toolchain = fenix.complete.toolchain;

          manifest = (lib.importTOML ./Cargo.toml).package;

          platform = makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        platform.buildRustPackage {
          meta.mainProgram = "nilla";
          pname = manifest.name;
          version = manifest.version;

          src = ./.;

          nativeBuildInputs = [ installShellFiles ];

          postInstall = ''
            installManPage ./target/release-tmp/build/nilla-*/out/nilla*
          '';

          cargoLock.lockFile = ./Cargo.lock;
        };
    };

    shells.default = config.shells.nilla-cli;
    shells.nilla-cli = {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

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
