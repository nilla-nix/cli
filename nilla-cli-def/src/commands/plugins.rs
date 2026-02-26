use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum PluginsCommands {
    /// List all available plugins
    List,
}

#[derive(Debug, Args)]
#[command(about = "Manage and list available plugins")]
pub struct PluginsArgs {
    #[command(subcommand)]
    pub command: PluginsCommands,
}

