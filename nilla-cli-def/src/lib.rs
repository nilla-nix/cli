pub mod commands;

use clap::{ArgAction, Parser, Subcommand};
use commands::{
    build::BuildArgs, completions::CompletionsArgs, run::RunArgs, shell::ShellArgs, show::ShowArgs,
};

#[derive(Parser, Debug)]
#[command(
	name = "nilla",
	version,
	long_about = None,
	after_help = commands::make_examples(&[
		("Run a package from a local Nilla project.", "run mypackage"),
		("Build a package from a Nilla project on GitHub.", "build mypackage --project github:myuser/myrepo"),
		("Start a development shell from a Nilla project in another directory.", "shell myshell --project ~/myproject"),
	]),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(
		long,
		short,
		help = "The nilla project to use (check Valid project sources in the man pages)",
		value_hint = clap::ValueHint::AnyPath,
		default_value = "./",
		global = true
	)]
    pub project: String,
    #[arg(
        long,
        short,
		action = ArgAction::Count,
        help = "The verbosity level to use",
        global = true
    )]
    pub verbose: u8,
    #[arg(
        long,
        short,
		action = ArgAction::SetTrue,
        help = "Quiet level of the program",
        global = true
    )]
    pub quiet: bool,
    #[arg(
        long,
		action = ArgAction::SetTrue,
        help = "Log any ran eval commands",
        global = true,
		default_value_t = false,
    )]
    pub show_eval_commands: bool,
    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub trailing_args: Vec<string>,
}

#[derive(Subcommand, Debug)]
#[command(allow_external_subcommands = true)]
pub enum Commands {
    Show(ShowArgs),
    Shell(ShellArgs),
    Run(RunArgs),
    Build(BuildArgs),
    #[command(alias = "completion")]
    Completions(CompletionsArgs),
    #[command(external_subcommand)]
    External(Vec<String>),
}
