use clap::Args;

#[derive(Debug, Args)]
#[command(
	about = "Start a development shell from a Nilla project",
	after_help = super::make_examples(&[
		("Start the default shell in a Nilla project on GitHub.", "shell --project github:myuser/myrepo"),
		("Start a specific shell in a local Nilla project.", "shell myshell")
	])
)]
pub struct ShellArgs {
    #[arg(
        help = "Name of the shell to start, if left empty it will use the default",
        default_value = "default"
    )]
    pub name: String,
    #[arg(help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
