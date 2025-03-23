{
  description = "The command line interface for Nilla.";

  outputs = inputs:
    let
      project = import ./nilla.nix;
    in
    {
      packages = {
        x86_64-linux = rec {
          nilla = project.packages.nilla-cli.result.x86_64-linux;
          default = nilla;
        };
      };
    };
}
