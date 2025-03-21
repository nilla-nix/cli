use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand};
use nilla_cli::commands::{
    build::BuildArgs, generate::GenerateArgs, nixos::NixosArgs, run::RunArgs, shell::ShellArgs,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, allow_external_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(
		long,
		short,
		help = "A path to the nilla project to use",
		value_hint = clap::ValueHint::FilePath,
		default_value = "./nilla.nix",
		global = true
	)]
    project: PathBuf,
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
    Generate(GenerateArgs),
}

fn main() {
    let cli = Cli::parse();

    // println!("Project: {:?}", cli.project.canonicalize());

    match &cli.command {
        Some(command) => match command {
            Commands::Shell(_args) => todo!(),
            Commands::Run(_args) => todo!(),
            Commands::Build(_args) => todo!(),
            Commands::Nixos(_args) => todo!(),
            Commands::Generate(args) => {
                nilla_cli::commands::generate::generate_cmd(args, &mut Cli::command())
            }
        },
        None => todo!(),
    }
}
