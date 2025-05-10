use log::{debug, error, info};
use serde_json::Value;

use crate::util::nix::{self, FixedOutputStoreEntry};

async fn determine_build_type(
    attribute: &str,
    file: &str,
    entry: FixedOutputStoreEntry,
) -> (String, String) {
    let hash = entry.hash;

    let store_path_name = nix::get_store_path_name(&entry.path);

    let file_str = entry.path.to_str().unwrap();

    let code = format!(
        "
	let
    source = builtins.path {{ path = \"{file_str}\"; sha256 = \"{hash}\"; name = \"{store_path_name}\"; }};
    project = import \"${{source}}/{file}\";
	in
	  project.{attribute}.name
	");

    let real_name_value = nix::evaluate(
        &code,
        nix::EvalOpts {
            json: true,
            impure: false,
        },
    )
    .await
    .unwrap();

    let real_name = match real_name_value {
        nix::EvalResult::Json(Value::String(s)) => s,
        _ => attribute.to_string(),
    };

    let split = attribute.split('.').collect::<Vec<&str>>();
    let build_type = split[0];

    match build_type {
        "systems" => ("system".to_string(), real_name),
        "shells" => ("shell".to_string(), real_name),
        "packages" => ("package".to_string(), real_name),
        _ => ("unknown attribute".to_string(), real_name),
    }
}

pub async fn build_cmd(cli: &nilla_cli_def::Cli, args: &nilla_cli_def::commands::build::BuildArgs) {
    debug!("Resolving project {}", cli.project);
    let Ok(project) = crate::util::project::resolve(&cli.project).await else {
        return error!("Could not find project {}", cli.project);
    };

    let entry = project.clone().get_entry();
    let mut subpath = project.clone().get_subpath();
    let mut path = project.clone().get_path().join(subpath.clone());

    debug!("Resolved project {path:?}");

    path.push("nilla.nix");
    subpath.push("nilla.nix");

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

    match nix::exists_in_project(
        subpath.to_str().unwrap_or("nilla.nix"),
        entry.clone(),
        &attribute,
    )
    .await
    {
        Ok(false) => {
            return error!("Attribute {attribute} does not exist in project {path:?}");
        }
        Err(e) => return error!("{e:?}"),
        _ => {}
    }

    let build_type = determine_build_type(
        attribute,
        path.iter().last().unwrap().to_str().unwrap(),
        entry.clone(),
    )
    .await;
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

    if let Err(e) = out {
        return error!("{:?}", e);
    };
}
