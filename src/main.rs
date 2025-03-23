use clap::{
    CommandFactory, Parser,
    builder::styling::{AnsiColor, Color::Ansi, Style},
};
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter, trace};
use nilla_cli_def::{Cli, Commands, commands::completions};
use tokio;

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

    trace!("Running {:?}", cli.command);

    match &cli.command {
        Some(command) => match command {
            Commands::Shell(args) => nilla::commands::shell::shell_cmd(&cli, args).await,
            Commands::Run(args) => nilla::commands::run::run_cmd(&cli, args).await,
            Commands::Build(args) => nilla::commands::build::build_cmd(&cli, args).await,
            Commands::Completions(args) => completions::completions_cmd(args, &mut Cli::command()),
            Commands::External(items) => println!("got external subcommand: {items:?}"),
        },
        None => {
            unreachable!();
        }
    };
    Ok(())
}
