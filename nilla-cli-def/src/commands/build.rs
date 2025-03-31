use clap::{ArgAction, Args};
#[derive(Debug, Args)]
#[command(
	about = "Build a package from a Nilla project",
	after_help = super::make_examples(&[
		("Build a package from a local Nilla project.", "build mypackage"),
		("Build a package from a Nilla project on GitHub.", "build mypackage --project github:myuser/myrepo"),
		("Build a package from a Nilla project in a tarball.", "build mypackage --project https://example.com/myproject.tar.gz"),
	])
)]
pub struct BuildArgs {
    #[arg(help = "Name of the shell to start, if left empty it will use the default")]
    pub name: Option<String>,
    #[arg(help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
    #[arg(
        long,
		action = ArgAction::SetTrue,
        help = "Link the build output to the current directory",
        default_value_t = false
    )]
    pub no_link: bool,
}

pub fn build_cmd(_cli: &crate::Cli, _args: &BuildArgs) {}
