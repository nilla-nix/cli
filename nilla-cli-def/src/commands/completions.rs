use clap::Command;
use clap_complete::{Shell, generate};

pub fn completions_cmd(args: &CompletionsArgs, cmd: &mut Command) {
    generate(
        args.shell,
        cmd,
        cmd.get_name().to_string(),
        &mut args.out.clone(),
    );
}

#[derive(Debug, clap::Args)]
#[command(about = "Generate autocompletions for your shell")]
pub struct CompletionsArgs {
    #[arg(long, short)]
    pub shell: Shell,
    #[clap(long, short, value_parser, default_value = "-")]
    pub out: clio::Output,
}
