pub mod commands;

use clap::{Parser, Subcommand};
use commands::{
    build::BuildArgs, completions::CompletionsArgs, nixos::NixosArgs, run::RunArgs,
    shell::ShellArgs,
};

#[derive(Parser, Debug)]
#[command(
	author,
	version,
	about,
	long_about = None,
	allow_external_subcommands = true,
	after_help = commands::make_examples(&[
		("Run a package from a local Nilla project.", "run mypackage"),
		("Build a package from a Nilla project on GitHub.", "build mypackage --project github:myuser/myrepo"),
		("Start a development shell from a Nilla project in another directory.", "shell myshell --project ~/myproject"),
		("Build and switch to a NixOS configuration in a local Nilla project.", "nixos switch mysystem")
	])
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(
		long,
		short,
		help = "The nilla project to use",
		value_hint = clap::ValueHint::AnyPath,
		default_value = "./",
		global = true
	)]
    project: String,
    #[arg(
        long,
        short,
        help = "The verbosity level to use",
        default_value_t = 0,
        global = true
    )]
    verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Shell(ShellArgs),
    Run(RunArgs),
    Build(BuildArgs),
    Nixos(NixosArgs),
    #[command(alias = "completion")]
    Completions(CompletionsArgs),
}
