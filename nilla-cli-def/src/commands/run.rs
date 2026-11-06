use clap::Args;

#[derive(Debug, Args)]
#[command(
	about = "Run a package's main program",
	after_help = super::make_examples(&[
		("Run the default package in a Nilla project on GitHub.", "run --project github:myuser/myrepo"),
		("Run a specific package in a local Nilla project.", "run mypackage"),
		("Run a specific binary from a package instead of its main program.", "run mypackage --bin mytool"),
		("Supply arguments to the package's main program using \"--\" followed by your arguments.", "run mypackage -- --my-arg")
	])
)]
pub struct RunArgs {
    #[arg(help = "Name of the program to run, if left empty it will use the default")]
    pub name: Option<String>,
    #[arg(help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
    #[arg(
        long,
        short,
        value_name = "BIN",
        help = "Binary in the package's bin directory to run, overrides meta.mainProgram"
    )]
    pub bin: Option<String>,
    #[arg(allow_hyphen_values = true, num_args = 0.., last = true)]
    pub remaining: Vec<String>,
}
