# 🍦 Nilla CLI

> Work with [Nilla](https://github.com/nilla-nix/nilla) projects with ease.

## Install without Flakes

You can install Nilla CLI in your NixOS, home-manager, or nix-darwin configuration.

```nix
# configuration.nix
{ pkgs, ... }:
let
  nilla-cli = import (builtins.fetchTarball {
    url = "https://github.com/nilla-nix/cli/archive/main.tar.gz";
    sha256 = "0000000000000000000000000000000000000000000000000000";
  });
  nilla-cli-package = nilla-cli.nilla.packages.nilla.${pkgs.system};
in
{
  environment.systemPackages = [
    nilla-cli-package
  ];
}
```

## Install with Flakes

You can add Nilla CLI as a Flake input.

```nix
# flake.nix
{
  inputs = {
    nilla-cli.url = "github:nilla-nix/cli";
  };

  outputs = { nilla-cli, ... }:
    let
      nilla-cli-package = inputs.nilla-cli.nilla.config.packages.nilla.build.x86_64-linux;
    in
      # Do something with the package.
      {};
}
```
