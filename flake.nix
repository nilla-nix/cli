{
  description = "The command line interface for Nilla.";

  outputs = inputs:
    let
      project = import ./nilla.nix;
    in
    {
      packages = builtins.mapAttrs
        (system: package: {
          default = package;
          nilla-cli = package;
        })
        project.packages.nilla-cli.result;
    };
}
