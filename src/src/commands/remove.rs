use std::process::Command;

use anyhow::Result;
use repo::config::{get_config, get_root_directory};

use crate::repo::parse_repos_with_error_log;

pub fn remove_repo(managed_path: &str) -> Result<()> {
    Command::new("rm")
        .args(vec!["--force", "--recursive", managed_path])
        .status()?;

    Ok(())
}

pub fn remove(repos: &[String]) -> Result<()> {
    for repo in parse_repos_with_error_log(&get_config()?, repos, true) {
        remove_repo(&repo.managed_path_name(&get_root_directory()?))?;

        println!("Removed {repo}.");
    }

    Ok(())
}
