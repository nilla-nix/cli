use clap::{CommandFactory, builder::styling::Style};

const HEADER_STYLE: Style = Style::new().bold().underline();

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR")
            .ok_or(std::io::ErrorKind::NotFound)
            .unwrap(),
    );

    let cmd = nilla_cli_def::Cli::command().after_long_help(format!(
        "
{HEADER_STYLE}Valid project sources{HEADER_STYLE:#}
  path

    Fetch a Nilla project from a file path. This follows the format:

      path:<path>

  git

    Fetch a Nilla project from a Git repository. This follows the format:

      git:<url>

    Optionally, additional customization can be applied using query parameters:

      git:<url>?rev=<rev>&ref=<ref>&submodules=true&dir=<dir>

  github

    Fetch a Nilla project from a GitHub repository. This follows the format:

      github:<owner>/<repo>

    Optionally, additional customization can be applied using query parameters:

      github:<owner>/<repo>?rev=<rev>&dir=<dir>

  gitlab

    Fetch a Nilla project from a GitLab repository. This follows the format:

      gitlab:<owner>/<repo>

    Optionally, additional customization can be applied using query parameters:

      gitlab:<owner>/<repo>?rev=<rev>&dir=<dir>

  tarball

    Fetch a Nilla project from a tarball. This follows the format:

      tarball:<url>

    The tarball source is also the default used when no other protocol matches. For
    example, the following are equivalent:

      tarball:http://example.com/project.tar.gz

      http://example.com/project.tar.gz
"
    ));

    let man = clap_mangen::Man::new(cmd.clone());

    let mans = cmd
        .get_subcommands()
        .map(|sc| clap_mangen::Man::new(sc.clone()));

    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join(man.get_filename()), buffer)?;

    for man in mans {
        let mut buffer: Vec<u8> = Default::default();
        man.render(&mut buffer)?;
        std::fs::write(
            out_dir.join(format!("nilla-{}", man.get_filename())),
            buffer,
        )?;
    }

    Ok(())
}
