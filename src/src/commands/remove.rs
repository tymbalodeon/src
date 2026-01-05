use std::process::Command;

use anyhow::Result;
use repo::config::{get_config, get_root_directory};

use crate::repo::parse_repos_with_error_log;

pub fn remove(repos: &[String]) -> Result<()> {
    for repo in parse_repos_with_error_log(&get_config()?, repos, true) {
        Command::new("rm")
            .args(vec![
                "--force",
                "--recursive",
                &repo.managed_path(&get_root_directory()?)?,
            ])
            .status()?;

        println!("Removed {repo}.");
    }

    Ok(())
}
