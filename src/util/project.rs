use anyhow::{anyhow, bail};
use log::{debug, trace};
use serde::Serialize;
use std::{borrow::Cow, path::PathBuf, str::FromStr};
use url::Url;

use crate::util::nix::{self, EvalResult};

#[derive(Debug)]
pub enum Source {
    Path { path: PathBuf },
    Git { info: GitInfo, path: PathBuf },
    Sourcehut { info: GitXInfo, path: PathBuf },
    Tarball { url: String, path: PathBuf },
}

impl Source {
    pub fn get_path(self) -> PathBuf {
        match self {
            Source::Path { path } => path,
            Source::Git { info: _, path } => path,
            Source::Sourcehut { info: _, path } => path,
            Source::Tarball { url: _, path } => path,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GitInfo {
    pub url: String,
    pub rev: Option<String>,
    pub r#ref: Option<String>,
    pub dir: Option<String>,
    pub submodules: bool,
}

#[derive(Debug)]
pub struct GitXInfo {
    pub owner: String,
    pub repo: String,
    pub rev: Option<String>,
    pub r#ref: Option<String>,
    pub dir: Option<String>,
    pub host: String,
    pub submodules: bool,
}

impl From<GitXInfo> for GitInfo {
    fn from(value: GitXInfo) -> Self {
        GitInfo {
            url: format!("git@{}:{}/{}.git", value.host, value.owner, value.repo),
            rev: value.rev,
            r#ref: value.r#ref,
            dir: value.dir,
            submodules: value.submodules,
        }
    }
}

fn remove_filename_from_path(mut path: PathBuf) -> PathBuf {
    if path.is_file() {
        trace!("Splicing off {:?}", path.file_name());
        path.pop();
    }
    path
}

async fn resolve_git(info: GitInfo) -> anyhow::Result<Source> {
    trace!("Resolving for {info:?}");
    let code = format!(
        "
		let
			info = builtins.fromJSON ''{}'';
		in
			builtins.fetchGit (
				{{ url = info.url; }}
				// (if info.rev != null then {{ rev = info.rev; }} else {{}})
				// (if info.ref != null then {{ ref = info.ref; }} else {{}})
				// (if info.submodules != null then {{ submodules = info.submodules; }} else {{}})
			)
	",
        serde_json::to_string(&info).unwrap()
    );

    let root = nix::evaluate(
        &code,
        nix::EvalOpts {
            impure: true,
            json: true,
        },
    )
    .await;

    let root_path = match root {
        Ok(EvalResult::Json(res)) => res.as_str().unwrap().to_string(),
        Ok(EvalResult::Raw(_)) => {
            bail!("Got raw, expected JSON");
        }
        _ => {
            bail!("{}", root.unwrap_err());
        }
    };

    let store_path = nix::realise(&PathBuf::from(root_path)).await;

    let Ok(paths) = store_path else {
        bail!("{}", store_path.unwrap_err());
    };

    let mut final_path = paths[0].clone();

    if info.dir.is_some() {
        final_path.push(info.dir.clone().unwrap())
    }

    return Ok(Source::Git {
        info,
        path: final_path,
    });
}

async fn resolve_tar(url: &str) -> anyhow::Result<Source> {
    let code = format!(
        "
		builtins.fetchTarball {{
			url = \"{url}\";
		}}
	"
    );

    let root = nix::evaluate(
        &code.trim(),
        nix::EvalOpts {
            impure: true,
            json: true,
        },
    )
    .await;

    let root_path = match root {
        Ok(EvalResult::Json(res)) => res.as_str().unwrap().to_string(),
        Ok(EvalResult::Raw(_)) => {
            bail!("Got raw, expected JSON");
        }
        _ => {
            bail!("{}", root.unwrap_err());
        }
    };

    let store_path = nix::realise(&PathBuf::from(root_path)).await;

    let Ok(paths) = store_path else {
        bail!("{}", store_path.unwrap_err());
    };

    return Ok(Source::Tarball {
        url: url.to_string(),
        path: paths[0].clone(),
    });
}

pub async fn resolve(uri: &str) -> anyhow::Result<Source> {
    trace!("Trying {uri} as local path");
    if uri.starts_with(".") || uri.starts_with("/") || uri.starts_with("~") {
        if let Ok(real_path) = {
            let Ok(path) = PathBuf::from_str(uri);
            path.canonicalize()
        } {
            debug!("Found path {}", real_path.display());
            return Ok(Source::Path {
                path: remove_filename_from_path(real_path),
            });
        } else {
            bail!("Could not find path {uri}");
        }
    }
    if uri.starts_with("path:") {
        if let Ok(real_path) = {
            let Ok(path) = PathBuf::from_str(&uri[4..]);
            path.canonicalize()
        } {
            debug!("Found path {}", real_path.display());
            return Ok(Source::Path {
                path: remove_filename_from_path(real_path),
            });
        } else {
            bail!("Could not find path {}", &uri[4..])
        };
    }

    trace!("Trying as URL");

    if uri.starts_with("git:") {
        trace!("matched as git");
        let url = Url::parse(uri).unwrap();
        let qps = url.query_pairs();
        let info = GitInfo {
            url: url.path().to_string(),
            rev: qps
                .clone()
                .find(|(k, _)| k == "rev")
                .map(|(_, v)| v.to_string()),
            r#ref: qps
                .clone()
                .find(|(k, _)| k == "ref")
                .map(|(_, v)| v.to_string()),
            dir: qps
                .clone()
                .find(|(k, _)| k == "dir")
                .map(|(_, v)| v.to_string()),
            submodules: qps
                .clone()
                .find(|(k, _)| k == "submodules")
                .unwrap_or(("".into(), "false".into()))
                .1
                .to_string()
                == "true",
        };
        return resolve_git(info).await;
    } else if uri.starts_with("github:") {
        trace!("matched as github");
        let url = Url::parse(&format!("github://{}", &uri[7..])).unwrap();
        let mut parsed = url
            .path_segments()
            .ok_or_else(|| anyhow!("cannot be base"))?;
        let owner = url.host().unwrap().to_string();
        let repo = parsed
            .next()
            .ok_or_else(|| anyhow!("could not get repo"))?
            .to_string();

        let qps = url.query_pairs();

        let info = GitXInfo {
            owner,
            repo,
            r#ref: qps
                .clone()
                .find(|(k, _)| k == "ref")
                .map(|(_, v)| v.to_string()),
            rev: qps
                .clone()
                .find(|(k, _)| k == "rev")
                .map(|(_, v)| v.to_string()),
            dir: qps
                .clone()
                .find(|(k, _)| k == "dir")
                .map(|(_, v)| v.to_string()),
            host: qps
                .clone()
                .find(|(k, _)| k == "host")
                .or(Some((Cow::from(""), Cow::from("github.com"))))
                .unwrap()
                .1
                .to_string(),
            submodules: qps
                .clone()
                .find(|(k, _)| k == "submodules")
                .unwrap_or(("".into(), "false".into()))
                .1
                .to_string()
                == "true",
        };
        return resolve_git(info.into()).await;
    } else if uri.starts_with("gitlab:") {
        trace!("matched as gitlab");
        let url = Url::parse(&format!("gitlab://{}", &uri[7..])).unwrap();
        let mut parsed = url
            .path_segments()
            .ok_or_else(|| anyhow!("cannot be base"))?;
        let owner = url.host().unwrap().to_string();
        let repo = parsed
            .next()
            .ok_or_else(|| anyhow!("could not get repo"))?
            .to_string();

        let qps = url.query_pairs();

        let info = GitXInfo {
            owner,
            repo,
            r#ref: qps
                .clone()
                .find(|(k, _)| k == "ref")
                .map(|(_, v)| v.to_string()),
            rev: qps
                .clone()
                .find(|(k, _)| k == "rev")
                .map(|(_, v)| v.to_string()),
            dir: qps
                .clone()
                .find(|(k, _)| k == "dir")
                .map(|(_, v)| v.to_string()),
            host: qps
                .clone()
                .find(|(k, _)| k == "host")
                .or(Some((Cow::from(""), Cow::from("gitlab.com"))))
                .unwrap()
                .1
                .to_string(),
            submodules: qps
                .clone()
                .find(|(k, _)| k == "submodules")
                .unwrap_or(("".into(), "false".into()))
                .1
                .to_string()
                == "true",
        };

        return resolve_git(info.into()).await;
    } else if uri.starts_with("tarball:") {
        trace!("matched as tarball");
        let mut minus_tar = uri[8..].to_string();
        if !minus_tar.starts_with("http://") && !minus_tar.starts_with("https://") {
            minus_tar = format!("http://{minus_tar}");
        }
        return resolve_tar(&minus_tar).await;
    } else if uri.starts_with("http://") || uri.starts_with("https://") {
        trace!("matched as http(s)");
        return resolve_tar(uri).await;
    } else {
        bail!("Could not parse URL Scheme for {uri}")
    }
}
