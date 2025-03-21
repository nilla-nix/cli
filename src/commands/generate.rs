use clap::Command;
use clap_complete::{Shell, generate};

pub fn generate_cmd(args: &GenerateArgs, cmd: &mut Command) {
    generate(
        args.shell,
        cmd,
        cmd.get_name().to_string(),
        &mut args.out.clone(),
    );
}

#[derive(Debug, clap::Args)]
#[command(about = "Generate autocompletions for your shell")]
pub struct GenerateArgs {
    #[arg(long, short)]
    shell: Shell,
    #[clap(long, short, value_parser, default_value = "-")]
    out: clio::Output,
}
