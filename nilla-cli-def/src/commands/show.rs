use clap::{Args};
#[derive(Debug, Args)]
#[command(
    about = "Show information about a Nilla project",
    after_help = super::make_examples(&[
        ("Show all information about a local Nilla project.", "show"),
        ("Show packages from a Nilla project on GitHub.", "show packages --project github:myuser/myrepo"),
        ("Show information about a specific package from a Nilla project in a tarball.", "show packages.mypackage --project https://example.com/myproject.tar.gz")
    ])
)]
pub struct ShowArgs {
    #[arg(help = "The item to show information about.")]
    pub name: Option<String>,
}

pub fn show_cmd(_cli: &crate::Cli, _args: &ShowArgs) {}
