use log::{debug, error, info};

use crate::util::nix;

fn determine_build_type(path: &str) -> (String, String) {
    if !path.contains(".") {
        ("Package".to_string(), path.to_string())
    } else {
        let split = path.split('.').collect::<Vec<&str>>();
        let build_type = split[0];

        match build_type {
            "systems" => ("System".to_string(), split[2].to_string()),
            "shells" => ("Shell".to_string(), split[1].to_string()),
            "packages" => ("Package".to_string(), split[1].to_string()),
            _ => ("Unknown attribute".to_string(), path.to_string()),
        }
    }
}

pub async fn build_cmd(cli: &nilla_cli_def::Cli, args: &nilla_cli_def::commands::build::BuildArgs) {
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

    let attribute = match &args.name {
        Some(name) => {
            if name.contains('.') {
                name
            } else {
                &format!("packages.\"{name}\".result.\"{system}\"")
            }
        }
        None => &format!("packages.default.result.\"{system}\""),
    };

    match nix::exists_in_project(&path, &attribute).await {
        Ok(false) => {
            return error!("Attribute {attribute} does not exist in project {path:?}");
        }
        Err(e) => return error!("{e:?}"),
        _ => {}
    }
    let build_type = determine_build_type(attribute);
    info!("Building {} {}", build_type.0, build_type.1);
    let out = nix::build(
        &path,
        &attribute,
        nix::BuildOpts {
            link: !args.no_link,
            report: true,
            system: &system,
        },
    )
    .await;

    let Ok(value) = out else {
        return error!("{:?}", out.err());
    };

    if args.print_out_paths {
        println!("{}", value.join("\n"))
    }
}
