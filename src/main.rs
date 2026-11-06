use anyhow::bail;
use clap::{
    CommandFactory, Parser,
    builder::styling::{AnsiColor, Color::Ansi, Style},
};
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter, debug, error, trace};
use nilla_cli_def::{Cli, Commands, commands::completions};
use nilla::commands::plugins;
use nilla::util::plugins::PluginInfo;

const B: Style = Style::new().bold();
const D: Style = Style::new().dimmed();
const R: Style = Style::new().fg_color(Some(Ansi(AnsiColor::Red)));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::White)
        .debug(Color::Magenta)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    // Check for --help flag - but only intercept if it's not for an external subcommand
    let args: Vec<String> = std::env::args().collect();
    let is_help_request = args.iter().any(|arg| arg == "--help" || arg == "-h");

    // If --help is requested, check if it's for an external subcommand (plugin)
    // If the first argument (after program name) is a known plugin, let it pass through
    if is_help_request && args.len() > 2 {
        if let Ok(plugins) = nilla::util::plugins::discover_plugins() {
            let first_arg = &args[1];
            if plugins.iter().any(|p| p.name == *first_arg) {
                // This is a help request for a plugin, let it pass through to external subcommand handling
                // Don't intercept it here
            } else {
                // This is a help request for nilla itself, customize command to include plugins
                let mut cmd = Cli::command();
                if !plugins.is_empty() {
                    let plugins_help = format_plugins_help(&plugins).await;
                    cmd = cmd.after_help(plugins_help);
                }
                // Let clap handle --help with our customized command
                let _ = cmd.print_long_help();
                std::process::exit(0);
            }
        } else {
            // No plugins found, just show nilla help
            let _ = Cli::command().print_long_help();
            std::process::exit(0);
        }
    } else if is_help_request {
        // Help requested but no additional args, show nilla help with plugins
        let mut cmd = Cli::command();
        if let Ok(plugins) = nilla::util::plugins::discover_plugins() {
            if !plugins.is_empty() {
                let plugins_help = format_plugins_help(&plugins).await;
                cmd = cmd.after_help(plugins_help);
            }
        }
        let _ = cmd.print_long_help();
        std::process::exit(0);
    }

    let cli = Cli::parse();
    let mut filter_level = match cli.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    if cli.quiet {
        filter_level = LevelFilter::Error;
    }

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "🍦 Nilla  {B}{}{B:#}  {}",
                colors.color(record.level()),
                match record.level() {
                    log::Level::Trace => format!("{D}{message}{D:#}"),
                    log::Level::Error => format!("{R}{message}{R:#}"),
                    _ => message.to_string(),
                }
            ));
        })
        .level(filter_level)
        .chain(
            fern::Dispatch::new()
                .filter(|f| f.level() == LevelFilter::Error)
                .chain(std::io::stderr()),
        )
        .chain(
            fern::Dispatch::new()
                .filter(|f| f.level() != LevelFilter::Error)
                .chain(std::io::stderr()),
        )
        .apply()?;
    let result = run_cli(cli).await;
    match result {
        Ok(c) => std::process::exit(c.unwrap_or(0)),
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}

async fn run_cli(cli: Cli) -> anyhow::Result<Option<i32>> {
    // Check for --help flag
    let args: Vec<String> = std::env::args().collect();
    let is_help_request = args.iter().any(|arg| arg == "--help" || arg == "-h");

    trace!("Running {:?}", cli.command);

    match &cli.command {
        Some(command) => match command {
            Commands::Show(args) => nilla::commands::show::show_cmd(&cli, args).await?,
            Commands::Shell(args) => nilla::commands::shell::shell_cmd(&cli, args).await?,
            Commands::Run(args) => nilla::commands::run::run_cmd(&cli, args).await?,
            Commands::Build(args) => nilla::commands::build::build_cmd(&cli, args).await?,
            Commands::Completions(args) => {
                let mut cmd = Cli::command();

                // Discover plugins and add them as subcommands for completions
                let discovered_plugins = nilla::util::plugins::discover_plugins().unwrap_or_default();

                // Collect plugin metadata first (async) to get their commands
                let mut plugin_metadata_vec = Vec::new();
                for plugin in &discovered_plugins {
                    let metadata = nilla::util::plugin_metadata::get_plugin_metadata(&plugin.path).await;
                    plugin_metadata_vec.push((plugin, metadata));
                }

                // Build plugin commands with their subcommands (sync)
                // Collect plugin info as static strings by leaking the allocations
                // This is acceptable for completion generation which is a one-time operation
                let plugin_commands: Vec<clap::Command> = plugin_metadata_vec.iter()
                    .map(|(plugin, metadata)| {
                        let name: &'static str = Box::leak(plugin.name.clone().into_boxed_str());
                        let about: &'static str = Box::leak(format!("Plugin: {}", plugin.path.display()).into_boxed_str());
                        let mut plugin_cmd = clap::Command::new(name).about(about);

                        // Add plugin commands as subcommands so clap_complete can generate completions
                        for cmd_info in &metadata.commands {
                            let cmd_name_static: &'static str = Box::leak(cmd_info.name.clone().into_boxed_str());
                            let cmd_desc_static: &'static str = Box::leak(cmd_info.description.clone().into_boxed_str());
                            plugin_cmd = plugin_cmd.subcommand(
                                clap::Command::new(cmd_name_static)
                                    .about(cmd_desc_static)
                            );
                        }

                        plugin_cmd
                    })
                    .collect();

                for plugin_cmd in plugin_commands {
                    cmd = cmd.subcommand(plugin_cmd);
                }

                // Generate base completions - this will include plugin commands
                completions::completions_cmd(args, &mut cmd);
            }
            Commands::Plugins(args) => {
                match &args.command {
                    nilla_cli_def::commands::plugins::PluginsCommands::List => {
                        plugins::plugins_list_cmd().await?;
                    }
                }
            }
            Commands::External(items) => {
                debug!("got external subcommand: {items:?}");
                let name = format!("nilla-{}", items[0]);
                let plugin_name = items[0].clone();

                match which::which(&name) {
                    Ok(path) => {
                        debug!("found external subcommand: {path:?}");

                        // Always capture output and transform "nilla-<plugin>" to "nilla <plugin>"
                        // This ensures consistent usage display regardless of how the plugin outputs help
                        let output = tokio::process::Command::new(&path)
                            .args(&items[1..])
                            .output()
                            .await?;

                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);

                        // Always transform "nilla-<plugin>" to "nilla <plugin>" in output
                        let transformed_stdout = stdout
                            .replace(&format!("nilla-{}", plugin_name), &format!("nilla {}", plugin_name));
                        let transformed_stderr = stderr
                            .replace(&format!("nilla-{}", plugin_name), &format!("nilla {}", plugin_name));

                        if !transformed_stdout.is_empty() {
                            print!("{}", transformed_stdout);
                        }
                        if !transformed_stderr.is_empty() {
                            eprint!("{}", transformed_stderr);
                        }

                        return Ok(output.status.code());
                    }
                    Err(_) => {
                        bail!("External subcommand not found: {name}");
                    }
                };
            }
        },
        None => {
            let help = Cli::command().render_long_help();
            println!("{}", help);

            // Append plugin information if any plugins are found
            if let Ok(plugins) = nilla::util::plugins::discover_plugins() {
                if !plugins.is_empty() {
                    println!("\n{}", format_plugins_help(&plugins).await);
                }
            }

            // If help was explicitly requested, exit successfully
            if is_help_request {
                return Ok(Some(0));
            }

            bail!("No subcommand found");
        }
    };

    Ok(Some(0))
}

/// Get plugin-specific completions by calling each plugin's completion command
async fn get_plugin_completions(
    plugins: &[PluginInfo],
    shell: clap_complete::Shell,
) -> anyhow::Result<String> {
    let mut all_completions = String::new();

    for plugin in plugins {
        // Get plugin metadata (cached, includes completion support check)
        let metadata = nilla::util::plugin_metadata::get_plugin_metadata(&plugin.path).await;

        if !metadata.supports_completions {
            continue;
        }

        // Try to get completions from the plugin
        let completion_output = tokio::process::Command::new(&plugin.path)
            .arg("completion")
            .arg(shell_to_string(shell))
            .output()
            .await;

        match completion_output {
            Ok(output) if output.status.success() => {
                let plugin_completions = String::from_utf8_lossy(&output.stdout);
                if !plugin_completions.trim().is_empty() {
                    // Add a comment to identify which plugin these completions are from
                    all_completions.push_str(&format!("\n# Completions for plugin: {}\n", plugin.name));
                    all_completions.push_str(&plugin_completions);
                }
            }
            _ => {
                // Plugin doesn't support completions or failed - skip silently
                debug!("Plugin {} does not support completions or failed", plugin.name);
            }
        }
    }

    Ok(all_completions)
}

/// Convert Shell enum to string for plugin commands
/// Only supports: bash, elvish, fish, powershell, zsh
fn shell_to_string(shell: clap_complete::Shell) -> &'static str {
    use clap_complete::Shell;
    match shell {
        Shell::Bash => "bash",
        Shell::Zsh => "zsh",
        Shell::Fish => "fish",
        Shell::PowerShell => "powershell",
        Shell::Elvish => "elvish",
        // Handle any other shells by falling back to bash
        _ => "bash",
    }
}

/// Format plugin information for help output
async fn format_plugins_help(plugins: &[PluginInfo]) -> String {
    use clap::builder::styling::Style;
    const HEADER_STYLE: Style = Style::new().bold().underline();

    let mut output = format!("{HEADER_STYLE}Available Plugins{HEADER_STYLE:#}\n");

    for plugin in plugins {
        // Get plugin metadata (cached)
        let metadata = nilla::util::plugin_metadata::get_plugin_metadata(&plugin.path).await;

        output.push_str(&format!("  {}\n", plugin.name));

        // Show usage if available
        if let Some(usage) = &metadata.usage {
            output.push_str(&format!("    {}\n", usage));
        }

        // Show commands if available
        if !metadata.commands.is_empty() {
            let cmd_names: Vec<String> = metadata.commands.iter().map(|c| c.name.clone()).collect();
            output.push_str(&format!("    Commands: {}\n", cmd_names.join(", ")));
        }

        // Show examples if available
        if !metadata.examples.is_empty() {
            output.push_str(&format!("    Examples: {}\n", metadata.examples.join("; ")));
        }

        output.push_str("\n");
    }

    output.push_str(&format!("  Use `nilla plugins list` to see all plugins with details.\n"));
    output.push_str(&format!("  Use `nilla <plugin> --help` to see plugin-specific help.\n"));

    output
}
