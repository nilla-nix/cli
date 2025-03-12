let
  nilla = import (builtins.fetchTarball {
    url = "https://github.com/nilla-nix/nilla/archive/main.tar.gz";
    sha256 = "1mhyfld3phd1vv4dxccvfkz37p0ffbykdzcvvc23cfrpvi4bcdnz";
  });
in
nilla.create {
  config = {
    inputs = {
      nixpkgs = {
        src = builtins.fetchTarball {
          url = "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz";
          sha256 = "0aa89pl1xs0kri9ixxg488n7riqi5n9ys89xqc0immyqshqc1d7f";
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

      package = { lib, buildNpmPackage, ... }:
        let
          pkg = lib.importJSON ./package.json;
        in
        buildNpmPackage {
          pname = "nilla";
          version = "v${pkg.version}";

          src = ./.;

          npmDepsHash = "sha256-M6hBtwaKCQMjLXeN+zUz/+jLZi0CIU+lHT/LmPhyEHg=";
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
