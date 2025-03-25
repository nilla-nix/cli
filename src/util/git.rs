use std::path::PathBuf;

use tokio::process::Command;

pub(crate) async fn get_untracked_files(repo: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .arg("ls-files")
        .arg("--others")
        .arg("--exclude-standard")
        .current_dir(repo)
        .output()
        .await?;
    let output = String::from_utf8(output.stdout)?;

    Ok(output.trim().lines().map(PathBuf::from).collect())
}
