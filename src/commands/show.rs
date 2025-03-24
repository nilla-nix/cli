use std::cmp::Ordering;

use log::{debug, error, info, trace};
use prettytable::{Attr, Cell, Row, Table, format};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::nix::{EvalOpts, EvalResult, FixedOutputStoreEntry, evaluate};

use colored::Colorize;

#[derive(Debug, Serialize, Deserialize)]
struct ExplainEntryData {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExplainEntry {
    name: String,
    description: String,

    data: ExplainEntryData,

    entries: Vec<ExplainEntry>,
}

fn show_entry(entry: ExplainEntry) {
    let name = format!(" {} ", entry.name);
    println!("{}", name.black().on_white().bold());

    if !entry.description.is_empty() {
        println!();
        println!("{}", entry.description);
    }

    if !entry.data.columns.is_empty() && !entry.data.rows.is_empty() {
        println!();

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        table.add_row(Row::new(
            entry
                .data
                .columns
                .iter()
                .map(|text| Cell::new(text).with_style(Attr::Bold))
                .collect(),
        ));

        for item in entry.data.rows {
            let mut vec: Vec<Cell> = item.iter().map(|text| Cell::new(text)).collect();
            let columns = entry.data.columns.len() as isize;
            let len = vec.len() as isize;
            let difference: isize = entry.data.columns.len() as isize - vec.len() as isize;

            match columns.cmp(&len) {
                Ordering::Greater => {
                    for _ in 0..difference {
                        vec.push(Cell::new(""));
                    }
                }
                Ordering::Less => {
                    for _ in 0..difference {
                        vec.pop();
                    }
                }
                _ => {}
            }

            table.add_row(Row::new(vec));
        }

        table.printstd();
    }

    if !entry.entries.is_empty() {
        for subentry in entry.entries {
            println!();
            show_entry(subentry);
        }
    }

    println!();
}

async fn show_attribute(file: &str, entry: FixedOutputStoreEntry, attribute: &str) {
    trace!("Getting explain entry for {attribute}");

    let file_str = entry.path.to_str().unwrap();

    let hash = entry.hash;

    let raw_entry = evaluate(
        &format!(
            "
    let
        source = builtins.path {{ path = \"{file_str}\"; sha256 = \"{hash}\"; }};
        project = import \"${{source}}/{file}\";
        attribute = \"{attribute}\";
    in
        project.explain.\"${{attribute}}\".result or null
        "
        ),
        EvalOpts {
            json: true,
            impure: false,
        },
    )
    .await;

    match raw_entry {
        Ok(EvalResult::Json(Value::Null)) => {}
        Ok(EvalResult::Json(value)) => {
            let serialized = value.to_string();

            let entry: ExplainEntry = match serde_json::from_str(serialized.as_str()) {
                Ok(e) => e,
                Err(e) => {
                    error!("Failed to parse explain entry for {attribute}: {e}");
                    return;
                }
            };

            trace!("Got explain entry for {attribute}: {entry:?}");
            show_entry(entry);
        }
        _ => {
            error!("Failed to get explain entry for {attribute}");
        }
    };
}

pub async fn show_cmd(cli: &nilla_cli_def::Cli, args: &nilla_cli_def::commands::show::ShowArgs) {
    debug!("Resolving project {}", cli.project);
    let Ok(project) = crate::util::project::resolve(&cli.project).await else {
        return error!("Could not find project {}", cli.project);
    };

    let entry = project.clone().get_entry();
    let mut path = project.get_path();

    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => return error!("File not found"),
        _ => {}
    }

    let file_str = entry.path.to_str().unwrap();

    let hash = entry.clone().hash;

    match &args.name {
        Some(name) => {
            let has_explainer = evaluate(
                &format!(
                    "
    let
        source = builtins.path {{ path = \"{file_str}\"; sha256 = \"{hash}\"; }};
        project = import \"${{source}}/nilla.nix\";
        attribute = \"{name}\";
    in
        project.explain ? ${{attribute}}
        "
                ),
                EvalOpts {
                    json: true,
                    impure: false,
                },
            )
            .await;

            match has_explainer {
                Ok(EvalResult::Json(Value::Bool(true))) => {
                    info!("Showing information about {} in {}", name, cli.project);
                    println!();
                    show_attribute("nilla.nix", entry.clone(), name.as_str()).await;
                }
                Ok(EvalResult::Json(Value::Bool(false))) => {
                    info!("No information available for {name}");
                }
                _ => {
                    error!("Failed to get info for {name}");
                }
            }
        }
        None => {
            debug!("Evaluating project");
            info!("Showing information about {}", cli.project);
            println!();

            let names_result = evaluate(
                &format!(
                    "
    let
        source = builtins.path {{ path = \"{file_str}\"; sha256 = \"{hash}\"; }};
        project = import \"${{source}}/nilla.nix\";
        reserved = [ \"assertions\" \"warnings\" \"extend\" \"explain\" ];
    in
        builtins.attrNames (builtins.removeAttrs project reserved)
        "
                ),
                EvalOpts {
                    json: true,
                    impure: false,
                },
            )
            .await;

            let Ok(EvalResult::Json(names)) = names_result else {
                return error!("Failed to get Nilla project attributes");
            };

            let Some(names_vec) = names.as_array() else {
                return error!("Failed to get Nilla project attributes");
            };

            let str_names = names_vec
                .iter()
                .map(|n| n.as_str().unwrap_or_default())
                .filter(|n| !n.is_empty())
                .collect::<Vec<&str>>();

            debug!("Got all names {str_names:?}");

            for name in str_names {
                show_attribute("nilla.nix", entry.clone(), name).await;
            }
        }
    };
}
