use std::path::PathBuf;

use log::{debug, error, info, trace};

use crate::util::nix;

pub async fn run_cmd(cli: &nilla_cli_def::Cli, args: &nilla_cli_def::commands::run::RunArgs) {
    debug!("Resolving project {}", cli.project);
    let rs = crate::util::project::resolve(&cli.project).await;

    let Ok(project) = rs else {
        return error!("{:?}", rs.unwrap_err());
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

    let (attribute, name) = match &args.name {
        Some(name) => (
            &format!("packages.\"{name}\".result.\"{system}\""),
            name.as_str(),
        ),
        None => (&format!("packages.default.result.\"{system}\""), "default"),
    };

    match nix::exists_in_project(&path, &attribute).await {
        Ok(false) => {
            return error!("Attribute {attribute} does not exist in project {path:?}");
        }
        Err(e) => return error!("{e:?}"),
        _ => {}
    }
    info!("Building package {name}");
    let out = nix::build(
        &path,
        &attribute,
        nix::BuildOpts {
            link: false,
            report: true,
            system: &system,
        },
    )
    .await;

    let Ok(value) = out else {
        return error!("{:?}", out.unwrap_err());
    };

    if value.len() == 0 {
        return error!("Package has no outputs");
    }

    let main_prog =
        nix::get_main_program(&path, &name, nix::GetMainProgramOpts { system: &system }).await;

    let Ok(main) = main_prog else {
        return error!("{:?}", main_prog.err());
    };

    let mut binary_path = PathBuf::from(value[0].clone());
    binary_path.push("bin");
    binary_path.push(main);
    trace!("Binary path: {:?}", binary_path.as_os_str());
    info!("Running Package {name}");

    let command_args = &args.remaining;
    debug!("With args: {}", command_args.join(" "));
    cargo_util::ProcessBuilder::new(binary_path)
        .args(command_args)
        .exec_replace()
        .unwrap();
    std::process::exit(0);
}
