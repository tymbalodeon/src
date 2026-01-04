use std::process::Command;

use anyhow::Result;
use repo::config::get_root_directory;

use crate::repo::get_repos;

pub fn remove(repos: &[String]) -> Result<()> {
    for repo in get_repos(repos) {
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
