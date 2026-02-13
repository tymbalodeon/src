use std::process::Command;

use anyhow::Result;
use colored::Colorize;
use inquire::{Confirm, MultiSelect};
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

    if repos.is_empty() {
        return Ok(());
    }

    let mut message = "Are you sure you want to proceed?".to_string();

    let repos = if repos.len() > 1 {
        let selected_repos = MultiSelect::new(
            "Select repositories to remove",
            repos
                .iter()
                .map(|repo| repo.managed_path_name(root_directory))
                .collect::<Vec<String>>(),
        )
        .prompt();

        if let Ok(selected_repos) = selected_repos {
            repos
                .into_iter()
                .filter(|repo| {
                    selected_repos
                        .contains(&repo.managed_path_name(root_directory))
                })
                .collect()
        } else {
            // TODO: exit with error?
            return Ok(());
        }
    } else if let Some(repo) = repos.first() {
        message = format!(
            "Are you sure you want to remove {}?",
            repo.managed_path_name(root_directory).cyan()
        );

        repos
    } else {
        // TODO: exit with error?
        return Ok(());
    };

    let remove = force
        || matches!(
            Confirm::new(&message)
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
