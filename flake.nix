{
  description = "The command line interface for Nilla.";

  outputs = inputs: {
    nilla = import ./nilla.nix;
  };
}
