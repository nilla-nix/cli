let
  pins = import ./npins;

  nilla = import pins.nilla;
in
nilla.create {
  config = {
    inputs = {
      nixpkgs = {
        src = pins.nixpkgs;

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
