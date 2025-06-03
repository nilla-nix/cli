use std::path::PathBuf;

use anyhow::bail;
use log::{debug, info, trace};

use crate::util::nix;

pub async fn run_cmd(
    cli: &nilla_cli_def::Cli,
    args: &nilla_cli_def::commands::run::RunArgs,
) -> anyhow::Result<()> {
    debug!("Resolving project {}", cli.project);
    let rs = crate::util::project::resolve(&cli.project).await;

    let Ok(project) = rs else {
        bail!("{:?}", rs.unwrap_err());
    };

    let entry = project.clone().get_entry();
    let mut subpath = project.clone().get_subpath();
    let mut path = project.clone().get_path().join(subpath.clone());

    debug!("Resolved project {path:?}");

    path.push("nilla.nix");
    subpath.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => bail!("File not found"),
        _ => {}
    }

    let system = match &args.system {
        Some(s) => s,
        _ => &match nix::get_system().await {
            Ok(s) => s,
            Err(e) => bail!("{e:?}"),
        },
    };

    let (attribute, name) = match &args.name {
        Some(name) => {
            if name.contains('.') {
                let sp = name.split('.').collect::<Vec<&str>>();
                (name, sp[1])
            } else {
                (
                    &format!("packages.\"{name}\".result.\"{system}\""),
                    name.as_str(),
                )
            }
        }
        None => (&format!("packages.default.result.\"{system}\""), "default"),
    };

    match nix::exists_in_project(
        subpath.to_str().unwrap_or("nilla.nix"),
        entry.clone(),
        attribute,
    )
    .await
    {
        Ok(false) => {
            bail!("Attribute {attribute} does not exist in project {path:?}");
        }
        Err(e) => bail!("{e:?}"),
        _ => {}
    }
    info!("Building package {name}");
    let out = nix::build(
        &path,
        attribute,
        nix::BuildOpts {
            link: false,
            report: true,
            system,
        },
    )
    .await;

    let Ok(value) = out else {
        bail!("{:?}", out.unwrap_err());
    };

    if value.is_empty() {
        bail!("Package has no outputs");
    }

    let main_prog = nix::get_main_program(
        path.iter().next_back().unwrap().to_str().unwrap(),
        entry.clone(),
        name,
        nix::GetMainProgramOpts { system },
    )
    .await;

    let Ok(main) = main_prog else {
        bail!("{:?}", main_prog.err());
    };

    let mut binary_path = PathBuf::from(value[0].clone());
    binary_path.push("bin");
    binary_path.push(main);
    trace!("Binary path: {:?}", path.as_os_str());
    info!("Running Package {name}");

    let command_args = &args.remaining;
    debug!("With args: {}", command_args.join(" "));
    cargo_util::ProcessBuilder::new(binary_path)
        .args(command_args)
        .exec_replace()
        .unwrap();

    Ok(())
}
