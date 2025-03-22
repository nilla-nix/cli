use clap::{CommandFactory, Parser};
use nilla_cli_def::{Cli, Commands, commands::completions};
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // println!("Project: {:?}", cli.project.canonicalize());

    match &cli.command {
        Some(command) => match command {
            Commands::Shell(_args) => todo!(),
            Commands::Run(_args) => todo!(),
            Commands::Build(_args) => todo!(),
            Commands::Completions(args) => completions::completions_cmd(args, &mut Cli::command()),
        },
        None => todo!(),
    }
}
