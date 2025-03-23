use log::{debug, error, info};

use crate::util::nix::{self, ShellOpts};

pub async fn shell_cmd(cli: &nilla_cli_def::Cli, args: &nilla_cli_def::commands::shell::ShellArgs) {
    debug!("Resolving project {}", cli.project);
    let Ok(project) = crate::util::project::resolve(&cli.project).await else {
        return error!("Could not find project {}", cli.project);
    };
    let mut path = project.get_path();
    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => return error!("File not found"),
        _ => {}
    }

    let system = match &args.system {
        Some(s) => s,
        _ => &match nix::get_system().await {
            Ok(s) => s,
            Err(e) => return error!("{e:?}"),
        },
    };

    let attribute = format!("shells.\"{}\".result.\"{system}\"", args.name);

    match nix::exists_in_project(&path, &attribute).await {
        Ok(false) => {
            return error!("Shell {attribute} does not exist in project {path:?}");
        }
        Err(e) => return error!("{e:?}"),
        _ => {}
    }

    info!("Entering shell {}", args.name);
    nix::shell(&path, &attribute, ShellOpts { system: &system });
}
