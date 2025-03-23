{
  description = "The command line interface for Nilla.";

  outputs = inputs:
    let
      project = import ./nilla.nix;
    in
    {
      packages = {
        x86_64-linux = rec {
          nilla = project.config.packages.nilla.result.x86_64-linux;
          default = nilla;
        };
      };
    };
}
