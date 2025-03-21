use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(about = "Manage a NixOS system in a Nilla project")]
pub struct NixosArgs {
    #[command(subcommand)]
    subcommand: NixosSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum NixosSubcommands {
    #[command(
        about = "Build and switch to a NixOS system",
        after_help = super::make_examples(&[("Switch to a NixOS configuration in a local Nilla project.", "nixos switch")])
    )]
    Switch(NixosSubcommandsArgs),
    #[command(
		about = "Build a NixOS system",
		after_help = super::make_examples(&[("Switch to a NixOS configuration in a local Nilla project.", "nixos build")])
	)]
    Build(NixosSubcommandsArgs),
    #[command(
		about = "Build a NixOS system VM", name = "build-vm",
		after_help = super::make_examples(&[("Build a NixOS VM in a Nilla project on GitHub.", "nixos build-vm --project github:myuser/myrepo")])
	)]
    BuildVm(NixosSubcommandsArgs),
    #[command(
		about = "Add a boot entry for a NixOS system",
		after_help = super::make_examples(&[("Add a boot entry for the \"mysystem\" NixOS configuration in a Nilla project.", "nixos boot mysystem")])
	)]
    Boot(NixosSubcommandsArgs),
    #[command(
		about = "Activate a NixOS system temporarily",
		after_help = super::make_examples(&[("Temporarily activate a NixOS configuration in a Nilla project.", "nixos test")])
	)]
    Test(NixosSubcommandsArgs),
}

#[derive(Debug, Args)]
pub struct NixosSubcommandsArgs {
    #[arg(value_hint = clap::ValueHint::Hostname, help = "The hostname of the system target")]
    name: Option<String>,
}
