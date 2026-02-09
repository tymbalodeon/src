use std::process::Command;

use anyhow::Result;
use inquire::Confirm;
use repo::config::{get_config, get_root_directory};

use crate::repo::parse_repos_with_error_log;

pub fn remove_repo(managed_path: &str) -> Result<()> {
    Command::new("rm")
        .args(vec!["--force", "--recursive", managed_path])
        .status()?;

    Ok(())
}

pub fn remove(
    repos: &[String],
    host: Option<&String>,
    owner: Option<&String>,
    me: bool,
    force: bool,
) -> Result<()> {
    let config = get_config()?;
    let owner = if me { config.owner.as_ref() } else { owner };
    let repos = parse_repos_with_error_log(&config, repos, host, owner, true)?;
    let root_directory = &get_root_directory()?;

    if !force && repos.len() > 0 {
        println!(
            "The following repositories would be removed:\n\n{}\n",
            repos
                .iter()
                .map(|repo| {
                    let path = repo.clone().managed_path_name(root_directory);

                    format!("- {path}")
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    let remove = force
        || matches!(
            Confirm::new("Are you sure you want to remove these?")
                .with_default(false)
                .with_help_message("This cannot be undone.")
                .prompt(),
            Ok(true)
        );

    if remove {
        for repo in repos {
            remove_repo(&repo.managed_path_name(root_directory))?;

            println!("Removed {repo}.");
        }
    }

    Ok(())
}
