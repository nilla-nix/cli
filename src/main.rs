use clap::{CommandFactory, Parser};
use nilla::Commands;
use tokio;

#[tokio::main]
async fn main() {
    let cli = nilla::Cli::parse();

    // println!("Project: {:?}", cli.project.canonicalize());

    match &cli.command {
        Some(command) => match command {
            Commands::Shell(_args) => todo!(),
            Commands::Run(_args) => todo!(),
            Commands::Build(_args) => todo!(),
            Commands::Nixos(_args) => todo!(),
            Commands::Generate(args) => {
                nilla::commands::generate::generate_cmd(args, &mut nilla::Cli::command())
            }
        },
        None => todo!(),
    }
}
