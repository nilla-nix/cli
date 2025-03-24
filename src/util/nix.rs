use std::{path::PathBuf, process::Stdio};

use anyhow::{Result, anyhow, bail};
use log::{debug, trace};
use serde_json::Value;
use tokio::process::Command;

pub struct EvalOpts {
    pub json: bool,
    pub impure: bool,
}

impl Default for EvalOpts {
    fn default() -> Self {
        Self {
            json: false,
            impure: false,
        }
    }
}

#[derive(Debug)]
pub enum EvalResult {
    Json(serde_json::Value),
    Raw(String),
}

pub async fn evaluate(code: &str, opts: EvalOpts) -> Result<EvalResult> {
    let mut args: Vec<&str> = vec![];
    args.append(&mut vec!["eval", "--show-trace"]);

    if opts.json {
        args.push("--json");
    }
    if opts.impure {
        args.push("--impure");
    }

    args.append(&mut vec!["--expr", &code]);

    debug!("Running nix {}", args.join(" "));
    let output = Command::new("nix").args(args).output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("nix eval failed\n{stderr}")
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if opts.json {
        Ok(EvalResult::Json(serde_json::from_str(stdout.trim())?))
    } else {
        Ok(EvalResult::Raw(stdout.trim().to_string()))
    }
}

pub async fn get_system() -> Result<String> {
    trace!("Getting system platform");
    match evaluate(
        "builtins.currentSystem",
        EvalOpts {
            json: true,
            impure: true,
        },
    )
    .await?
    {
        EvalResult::Json(value) => match &value {
            serde_json::Value::String(s) => {
                debug!("Got {s}");
                return Ok(value.as_str().unwrap().to_string());
            }
            _ => bail!("Got: '{value:?}', Expected String"),
        },
        EvalResult::Raw(v) => bail!("Somehow returned raw with value: '{v}'"),
    };
}

pub async fn realise(path: &PathBuf) -> Result<Vec<PathBuf>> {
    trace!("Realising {path:?}");
    let output = Command::new("nix-store")
        .args(["--realise", path.to_str().unwrap()])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("nix-store realise failed:\n{stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout
        .lines()
        .map(|i| PathBuf::from(i))
        .collect::<Vec<PathBuf>>())
}

pub struct BuildOpts<'a> {
    pub link: bool,
    pub report: bool,
    pub system: &'a str,
}

pub async fn build(file: &PathBuf, name: &str, opts: BuildOpts<'_>) -> Result<Vec<String>> {
    let mut args = vec!["build"];
    if !opts.link {
        args.push("--no-link");
    }
    if opts.report {
        args.push("--print-out-paths");
    }
    args.push("-f");
    args.push(file.to_str().unwrap());
    if opts.system != "" {
        args.push("--system");
        args.push(opts.system);
    };
    args.push(&name);
    trace!("Running nix {}", args.join(" "));
    let cmd = Command::new("nix")
        .stdout(Stdio::piped())
        .args(args)
        .spawn()?;

    return Ok(
        String::from_utf8_lossy(&cmd.wait_with_output().await.unwrap().stdout)
            .lines()
            .map(|s| s.to_owned())
            .collect(),
    );
}

pub struct ShellOpts<'a> {
    pub system: &'a str,
}

pub fn shell(file: &PathBuf, name: &str, opts: ShellOpts<'_>) {
    let mut args = vec![file.to_str().unwrap()];
    if opts.system != "" {
        args.push("--system");
        args.push(opts.system);
    }
    args.push("-A");
    args.push(name);

    debug!("Replacing process with nix-shell {name}");
    cargo_util::ProcessBuilder::new("nix-shell")
        .args(&args)
        .exec_replace()
        .unwrap();
    std::process::exit(0);
}

pub struct GetMainProgramOpts<'a> {
    pub system: &'a str,
}

pub async fn get_main_program(
    file: &PathBuf,
    name: &str,
    opts: GetMainProgramOpts<'_>,
) -> Result<String> {
    let file_str = file.to_str().unwrap();
    let main = evaluate(
        &format!(
            "
			let
				project = import (builtins.toPath \"{file_str}\");
				system = \"{}\";
				name = \"{name}\";
			in
				project.packages.${{name}}.result.${{system}}.meta.mainProgram or name
			",
            if opts.system == "" {
                get_system().await?
            } else {
                opts.system.to_string()
            }
        ),
        EvalOpts {
            json: true,
            impure: true,
        },
    )
    .await?;

    match main {
        EvalResult::Json(Value::String(s)) => Ok(s),
        _ => bail!("Somehow got raw or wrong type"),
    }
}

pub async fn exists_in_project(file: &PathBuf, name: &str) -> Result<bool> {
    let file_str = file.to_str().unwrap();
    let code = if name.contains('.') {
        let parts = name.split('.').collect::<Vec<&str>>();
        let last = parts.last().ok_or(anyhow!("How did we get here"))?;
        let init = &parts[0..parts.len() - 1].join(".");
        format!(
            "
		let
			project = import (builtins.toPath \"{file_str}\");
		in
			(project.{init} or {{}}) ? {last}
		"
        )
    } else {
        format!(
            "
		let
			project = import (builtins.toPath \"{file_str}\");
		in
			project ? {name}
		"
        )
    };

    let result = evaluate(
        &code,
        EvalOpts {
            json: true,
            impure: true,
        },
    )
    .await?;

    match result {
        EvalResult::Json(Value::Bool(b)) => Ok(b),
        _ => bail!("Got a non boolean result {result:?}"),
    }
}
