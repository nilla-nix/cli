let
  pins = import ./npins;

  nilla = import pins.nilla;
in
nilla.create {
  config = {
    inputs = {
      nixpkgs = {
        src = builtins.fetchTarball {
          url = "https://github.com/NixOS/nixpkgs/archive/c80f6a7e10b39afcc1894e02ef785b1ad0b0d7e5.tar.gz";
          sha256 = "1sfb9g6fmyfligcsd1rmkamfqvy8kgn3p0sy8ickf6swi1zdbf0b";
        };

        loader = "legacy";

        settings = {
          args = {
            system = "x86_64-linux";
          };
        };
      };
    };

    packages.nilla = {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      package = { lib, buildNpmPackage, makeWrapper, nixos-rebuild-ng, ... }:
        let
          pkg = lib.importJSON ./package.json;
        in
        buildNpmPackage {
          pname = "nilla";
          version = "v${pkg.version}";

          src = ./.;

          npmDepsHash = "sha256-M6hBtwaKCQMjLXeN+zUz/+jLZi0CIU+lHT/LmPhyEHg=";

          nativeBuildInputs = [ makeWrapper ];

          postInstall = ''
            wrapProgram $out/bin/nilla --prefix PATH : ${nixos-rebuild-ng}/bin
          '';
        };
    };

    shells.default = {
      systems = [ "x86_64-linux" ];

      shell = { mkShell, nodejs, nixos-rebuild-ng, ... }:
        mkShell {
          packages = [
            nodejs
            nixos-rebuild-ng
          ];
        };
    };
  };
}
