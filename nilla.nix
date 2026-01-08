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

            # Install shell completions that dynamically discover plugins
            mkdir -p $out/share/zsh/site-functions
            cp ${./completions/_nilla.zsh} $out/share/zsh/site-functions/_nilla
            mkdir -p $out/share/bash-completion/completions
            cp ${./completions/nilla.bash} $out/share/bash-completion/completions/nilla
            mkdir -p $out/share/fish/vendor_completions.d
            cp ${./completions/nilla.fish} $out/share/fish/vendor_completions.d/nilla.fish
            mkdir -p $out/share/elvish/lib
            cp ${./completions/nilla.elv} $out/share/elvish/lib/nilla.elv
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
