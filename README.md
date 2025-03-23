# 🍦 Nilla CLI

> Work with [Nilla](https://github.com/nilla-nix/nilla) projects with ease.

## Install with Nilla

You can add the Nilla CLI to your Nilla project and access using the following code:

```nix
# In any module of your project.
{ config }:
let
    nilla-cli-package = config.inputs.nilla-cli.packages.nilla.x86_64-linux;
in
{
    config = {
        inputs.nilla-cli.src = builtins.fetchTarball {
            url = "https://github.com/nilla-nix/cli/archive/main.tar.gz";
            sha256 = "0000000000000000000000000000000000000000000000000000";
        };

        # Do something with the package.
    };
}
```

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
  nilla-cli-package = nilla-cli.packages.nilla.result.${pkgs.system};
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
      nilla-cli-package = nilla-cli.packages.x86_64-linux.nilla-cli;
    in
      # Do something with the package.
      {};
}
```

## Run with Flakes

You can run the Nilla CLI directly via Flakes.

```bash
# Place any arguments you want to provide to Nilla CLI after the --
nix run github:nilla-nix/cli -- --help
```
